use csv;
use params;
use rand;
use rand::Rng;
use std::fs::File;
use std::slice::Iter;

#[derive(Copy, Clone)]
pub struct DataRecord{
    pub features: [f32; params::dataset::N_FEATURES as usize],
    pub class: bool,
}

impl DataRecord{
    fn new_blank()->DataRecord{
        DataRecord{
            features: [0.0; params::dataset::N_FEATURES as usize],
            class: false,
        }
    }
}


pub trait DataSet{
    fn record_iter(&self) -> Iter<DataRecord>;
    fn size(&self) -> usize;
}

pub struct FullDataSet{
    pub records: [DataRecord; params::dataset::N_SAMPLES],
}

pub struct TestDataSet {
    pub records: [DataRecord; params::dataset::TEST_DATA_SET_SIZE],
}

pub struct ValidationSet{
    pub records: [DataRecord; params::dataset::FOLD_SIZE],
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

impl FullDataSet{

    pub fn new(data_file: &str) -> FullDataSet{

        let mut records = [DataRecord::new_blank(); params::dataset::N_SAMPLES];
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

                let mut features = [0.0f32; params::dataset::N_FEATURES as usize];

                for (j, next_entry) in result_iter.enumerate() {
                    match next_entry.parse::<f32>() {
                        Ok(entry) => features[j] = entry,
                        Err(_) => features[j] = params::params::NA_TOKEN
                    }
                }
                records[i] = DataRecord{features, class};
            }
        }

        FullDataSet{
            records,
        }
    }
}


pub fn gen_partitions() -> Vec<Partition> {
    let n_fold = 5;
    let mut rng = rand::thread_rng();

    let mut chosen = Vec::with_capacity(params::dataset::N_SAMPLES);
    let mut partitions = Vec::with_capacity(n_fold);

    for _ in 0..n_fold{
        let mut cases = Vec::with_capacity(params::dataset::N_POS_FOLD);
        let mut controls = Vec::with_capacity(params::dataset::N_NEG_FOLD);
        let mut tries = 0;

        while (cases.len() < params::dataset::N_POS_FOLD || controls.len() < params::dataset::N_NEG_FOLD){
            let chioce = rng.gen_range(0, params::dataset::N_SAMPLES);
            tries += 1;

            if !chosen.contains(&chioce){  //selected a sample not yet chosen
                let is_case = is_case(chioce);

                if is_case && cases.len() < params::dataset::N_POS_FOLD{
                    cases.push(chioce.clone());
                    chosen.push(chioce);
                    tries = 0;
                }
                else if !is_case && controls.len() < params::dataset::N_NEG_FOLD{
                    controls.push(chioce.clone());
                    chosen.push(chioce);
                    tries = 0;
                }
            }

            if tries >= params::params::DUPLICATE_TIME_OUT*2{
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
    if n >= params::dataset::POS_SAMPLE_RNG.start && n < params::dataset::POS_SAMPLE_RNG.end {true}
    else if n >= params::dataset::NEG_SAMPLE_RNG.start && n < params::dataset::NEG_SAMPLE_RNG.end {false}
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

    pub fn new_rand_partition() -> DataSetManager{
        DataSetManager{partitions:gen_partitions(), current_partition:0}
    }

    pub fn next_set(&mut self) -> Option<(TestDataSet, ValidationSet)>{

        if self.current_partition >= params::dataset::N_FOLDS{return None}

        let mut test_records = [DataRecord::new_blank(); params::dataset::TEST_DATA_SET_SIZE];
        let mut cv_records = [DataRecord::new_blank(); params::dataset::FOLD_SIZE];

        let mut test_dataset_i = 0;
        let mut cv_dataset_i = 0;

        let full_set = FullDataSet::new(params::dataset::DATA);

        for (partition_i, partition) in self.partitions.iter().enumerate() {

            if partition_i == self.current_partition as usize { // cv
                for sample_i in partition.cases.iter() {
                    cv_records[cv_dataset_i] = full_set.records[*sample_i];
                    cv_dataset_i += 1;
                }
                for sample_i in partition.controls.iter() {
                    cv_records[cv_dataset_i] = full_set.records[*sample_i];
                    cv_dataset_i += 1;
                }
            }
            else { //test
                for sample_i in partition.cases.iter() {
                    test_records[test_dataset_i] = full_set.records[*sample_i];
                    test_dataset_i += 1;
                }
                for sample_i in partition.controls.iter() {
                    test_records[test_dataset_i] = full_set.records[*sample_i];
                    test_dataset_i += 1;
                }
            }
        }
        self.current_partition += 1;
        Some((TestDataSet { records:test_records,},
         ValidationSet{ records:cv_records,}))
    }
}

