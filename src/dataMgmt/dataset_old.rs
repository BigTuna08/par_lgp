use csv;
use params;
use rand;
use rand::Rng;
use std::fs::File;
use std::slice::Iter;

#[derive(Copy, Clone)]
pub struct DataRecord{
    pub features: [f32; params::N_FEATURES as usize],
    pub class: bool,
}

impl DataRecord{
    fn new_blank()->DataRecord{
        DataRecord{
            features: [0.0; params::N_FEATURES as usize],
            class: false,
        }
    }
}


pub trait DataSet{
    fn record_iter(&self) -> Iter<DataRecord>;
    fn size(&self) -> usize;
}

pub struct FullDataSet{
    pub records: [DataRecord; params::N_SAMPLES],
    pub partitions: Option<Vec<Partition>>,
}

pub struct PartitionedDataSet{
    pub test: TestDataSet,
    pub cv: ValidationSet,
    current_partition: u8,
}


pub struct TestDataSet {
    pub records: [DataRecord; params::TEST_DATA_SET_SIZE],
}


pub struct ValidationSet{
    pub records: [DataRecord; params::FOLD_SIZE],
}


impl DataSet for FullDataSet{
    fn record_iter(&self) -> Iter<DataRecord>{
        self.records.iter()
    }
    fn size(&self) -> usize{
        self.records.len()
    }
}

impl DataSet for TestDataSet{
    fn record_iter(&self) -> Iter<DataRecord>{
        self.records.iter()
    }

    fn size(&self) -> usize{
        self.records.len()
    }
}

impl DataSet for ValidationSet{
    fn record_iter(&self) -> Iter<DataRecord>{
        self.records.iter()
    }

    fn size(&self) -> usize{
        self.records.len()
    }
}

impl PartitionedDataSet{ //test these methods!!

    pub fn next(&mut self) ->bool{
        self.current_partition += 1;
        if self.current_partition >= params::N_FOLDS {return false}

        else {
            self.next_partition();
            return true
        }
    }

    fn next_partition(&mut self){
        let mut new_cv = [DataRecord::new_blank(); params::FOLD_SIZE];
        for i in 0..params::FOLD_SIZE{
            new_cv[i] = self.test.records[i];
        }


        for i in 0..params::TEST_DATA_SET_SIZE {
            if i+params::FOLD_SIZE < self.test.size(){
                self.test.records[i] = self.test.records[i+params::FOLD_SIZE];
            }
            else {
                self.test.records[i] = self.cv.records[i - self.test.size()];
            }
        }
        self.cv.records = new_cv;

    }
}


impl FullDataSet{

    pub fn to_partioned_set(&self, trial_n: usize) -> PartitionedDataSet {

        let mut test_records = [DataRecord::new_blank(); params::TEST_DATA_SET_SIZE];
        let mut cv_records = [DataRecord::new_blank(); params::FOLD_SIZE];

        let mut test_dataset_i = 0;
        let mut cv_dataset_i = 0;

        match self.partitions {
            None => panic!("Error, data not partitioned!"),
            Some(ref partitions) => {
                for (partition_i, partition) in partitions.iter().enumerate() {

                    if partition_i == trial_n { // cv
                        for sample_i in partition.cases.iter() {
                            cv_records[cv_dataset_i] = self.records[*sample_i];
                            cv_dataset_i += 1;
                        }
                        for sample_i in partition.controls.iter() {
                            cv_records[cv_dataset_i] = self.records[*sample_i];
                            cv_dataset_i += 1;
                        }
                    }
                    else { //test
                        for sample_i in partition.cases.iter() {
                            test_records[test_dataset_i] = self.records[*sample_i];
                            test_dataset_i += 1;
                        }
                        for sample_i in partition.controls.iter() {
                            test_records[test_dataset_i] = self.records[*sample_i];
                            test_dataset_i += 1;
                        }
                    }
                }
            }
        }
        assert_eq!(test_dataset_i, params::TEST_DATA_SET_SIZE);
        assert_eq!(cv_dataset_i, params::FOLD_SIZE);

        PartitionedDataSet{
            test: TestDataSet { records:test_records,},
            cv: ValidationSet{ records:cv_records,},
            current_partition:0,
        }
    }


    pub fn get_parts(&self, trial_n: usize) -> (TestDataSet, ValidationSet) {
        let mut test_records = [DataRecord::new_blank(); params::TEST_DATA_SET_SIZE];
        let mut cv_records = [DataRecord::new_blank(); params::FOLD_SIZE];

        let mut test_dataset_i = 0;
        let mut cv_dataset_i = 0;

        match self.partitions {
            None => panic!("Error, data not partitioned!"),
            Some(ref partitions) => {
                for (partition_i, partition) in partitions.iter().enumerate() {

                    if partition_i == trial_n { // cv
                        for sample_i in partition.cases.iter() {
                            cv_records[cv_dataset_i] = self.records[*sample_i];
                            cv_dataset_i += 1;
                        }
                        for sample_i in partition.controls.iter() {
                            cv_records[cv_dataset_i] = self.records[*sample_i];
                            cv_dataset_i += 1;
                        }
                    }
                        else { //test
                            for sample_i in partition.cases.iter() {
                                test_records[test_dataset_i] = self.records[*sample_i];
                                test_dataset_i += 1;
                            }
                            for sample_i in partition.controls.iter() {
                                test_records[test_dataset_i] = self.records[*sample_i];
                                test_dataset_i += 1;
                            }
                        }
                }
            }
        }
        assert_eq!(test_dataset_i, params::TEST_DATA_SET_SIZE);
        assert_eq!(cv_dataset_i, params::FOLD_SIZE);

        (TestDataSet { records:test_records,},
            ValidationSet{ records:cv_records,})
    }


    pub fn new(data_file: &str) -> FullDataSet{

        let mut records = [DataRecord::new_blank(); params::N_SAMPLES];
        let f = File::open(data_file).unwrap();
        let mut csv_rdr = csv::Reader::from_reader(f);

        for (i,result) in csv_rdr.records().enumerate() {
            if let Ok(result) = result{
                let mut result_iter = result.iter();

                result_iter.next(); //skip first 2
                result_iter.next();

                let class = match result_iter.next().unwrap() {
                    "0" => false,
                    "1" => true,
                    _ => panic!("Invalid classification field!!")
                };

                let mut features = [0.0f32; params::N_FEATURES as usize];

                for (j, next_entry) in result_iter.enumerate() {
                    match next_entry.parse::<f32>() {
                        Ok(entry) => features[j] = entry,
                        Err(_) => features[j] = params::NA_TOKEN
                    }
                }
                records[i] = DataRecord{features, class};
            }
        }

        let mut ds = FullDataSet{
            records, partitions:None,
        };
        ds.set_partitions();
        ds
    }

}


impl FullDataSet{
    fn set_partitions(&mut self){
        match self.partitions {
            Some(_) => panic!("Data has already been partitioned!!!"),
            None => self.partitions = Some(gen_partitions()),
        };
    }
}

pub fn gen_partitions() -> Vec<Partition> {
    let n_fold = 5;
    let mut rng = rand::thread_rng();

    let mut chosen = Vec::with_capacity(params::N_SAMPLES);
    let mut partitions = Vec::with_capacity(n_fold);

    for _ in 0..n_fold{
        let mut cases = Vec::with_capacity(params::N_POS_FOLD);
        let mut controls = Vec::with_capacity(params::N_NEG_FOLD);
        let mut tries = 0;

        while (cases.len() < params::N_POS_FOLD || controls.len() < params::N_NEG_FOLD){
            let chioce = rng.gen_range(0, params::N_SAMPLES);
            tries += 1;

            if !chosen.contains(&chioce){  //selected a sample not yet chosen
                let is_case = is_case(chioce);

                if is_case && cases.len() < params::N_POS_FOLD{
                    cases.push(chioce.clone());
                    chosen.push(chioce);
                    tries = 0;
                }
                else if !is_case && controls.len() < params::N_NEG_FOLD{
                    controls.push(chioce.clone());
                    chosen.push(chioce);
                    tries = 0;
                }
            }

            if tries >= params::DUPLICATE_TIME_OUT*2{
                panic!("Error generating data set!");
            }
        }
        partitions.push(Partition{cases, controls});
    }
    partitions
}


pub fn get_headers(data_file: &str) -> Vec<String> {
    match File::open(data_file){
        Ok(f) => {
            match csv::Reader::from_reader(f).headers() {
                Ok(headers) => {
                    let mut full_iter = headers.iter();
                    full_iter.next();
                    full_iter.next();
                    full_iter.next();
                    full_iter.map(|x|  x.to_string()).collect()
                }
                Err(e) =>panic!("couldnt get file headers!! error is :{:?}", e)
            }
        }
        Err(e) => panic!("couldnt open file to get headers!! error is :{:?}", e)
    }
}


fn is_case(n: usize)->bool{
    if n >= params::POS_SAMPLE_RNG.start && n < params::POS_SAMPLE_RNG.end {true}
    else if n >= params::NEG_SAMPLE_RNG.start && n < params::NEG_SAMPLE_RNG.end {false}
    else {panic!("outside data range! {}", n)}
}

#[derive(Clone)]
pub struct Partition{
    cases: Vec<usize>,
    controls: Vec<usize>,
}


pub struct DataSetManager{
    partitions: Vec<Partition>,
    current_partition: u8,
}

impl DataSetManager{

    pub fn new(partitions: Vec<Partition>) -> DataSetManager{
        DataSetManager{partitions, current_partition:0}
    }

    pub fn next_set(&mut self) -> (TestDataSet, ValidationSet){

    }
}

