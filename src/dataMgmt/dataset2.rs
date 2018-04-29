use csv;
use params;
use rand;
use rand::Rng;
use std::fs::File;
use std::slice::Iter;

pub trait DataSet{
    fn feature_iter(&self) -> Iter<[f32; params::dataset::N_FEATURES as usize]>;
    fn is_case(&self, usize) -> bool;
    fn size(&self) -> usize;
}

pub struct FullDataSet{
    pub features: [[f32; params::dataset::N_FEATURES as usize]; params::dataset::N_SAMPLES],
    pub classes: [bool; params::dataset::N_SAMPLES],
    pub partitions: Option<Vec<Partition>>,
}

pub struct PartitionedDataSet{
    pub test: TestDataSet,
    pub cv: ValidationSet,
}


pub struct TestDataSet {
    pub features: [[f32; params::dataset::N_FEATURES as usize]; params::params::TEST_DATA_SET_SIZE],
    pub classes: [bool; params::params::TEST_DATA_SET_SIZE],
}


pub struct ValidationSet{
    pub features: [[f32; params::dataset::N_FEATURES as usize]; params::params::FOLD_SIZE],
    pub classes: [bool; params::params::FOLD_SIZE],
}


impl TestDataSet {
    pub fn get_dataset(trial_n: usize, partitions: &Vec<Partition>, data_file: &str) -> TestDataSet {
        let mut full = FullDataSet::new_with_partitions(data_file, partitions);
        full.to_test_dataset(trial_n)
    }
}


impl ValidationSet{
    pub fn get_validation_dataset(trial_n: usize, partitions: &Vec<Partition>, data_file: &str) -> ValidationSet{
        let mut full = FullDataSet::new_with_partitions(data_file, partitions);
        full.to_cv_set(trial_n)
    }
}


impl DataSet for FullDataSet{
    fn feature_iter(&self) -> Iter<[f32; params::dataset::N_FEATURES as usize]>{
        self.features.iter()
    }
    fn is_case(&self, sample_n: usize) -> bool{
        if sample_n >= params::dataset::N_SAMPLES{
            panic!("Outside sample range!!");
        }
        self.classes[sample_n]
    }
    fn size(&self) -> usize{
        self.classes.len()
    }
}


impl DataSet for TestDataSet{
    fn feature_iter(&self) -> Iter<[f32; params::dataset::N_FEATURES as usize]>{
        self.features.iter()
    }
    fn is_case(&self, sample_n: usize) -> bool{
        if sample_n >= params::dataset::N_SAMPLES{
            panic!("Outside sample range!!");
        }
        self.classes[sample_n]
    }
    fn size(&self) -> usize{
        self.classes.len()
    }
}


impl DataSet for ValidationSet{
    fn feature_iter(&self) -> Iter<[f32; params::dataset::N_FEATURES as usize]>{
        self.features.iter()
    }
    fn is_case(&self, sample_n: usize) -> bool{
        if sample_n >= params::dataset::N_SAMPLES{
            panic!("Outside sample range!!");
        }
        self.classes[sample_n]
    }
    fn size(&self) -> usize{
        self.classes.len()
    }
}


impl FullDataSet{


    pub fn to_partioned_set(&self, trial_n: usize) -> PartitionedDataSet {
        let mut test_features = [[0.0f32; params::dataset::N_FEATURES as usize]; params::params::TEST_DATA_SET_SIZE];
        let mut test_classes = [false; params::params::TEST_DATA_SET_SIZE];

        let mut dataset_i = 0;

        match self.partitions {
            None => panic!("Error, data not partitioned!"),
            Some(ref partitions) => {
                for (partition_i, partition) in partitions.iter().enumerate() {
                    if partition_i == trial_n {continue;}

                    for sample_i in partition.cases.iter() {
                        test_features[dataset_i] = self.features[*sample_i];
                        test_classes[dataset_i] = self.classes[*sample_i];
                        dataset_i += 1;
                    }
                    for sample_i in partition.controls.iter() {
                        test_features[dataset_i] = self.features[*sample_i];
                        test_classes[dataset_i] = self.classes[*sample_i];
                        dataset_i += 1;
                    }
                }
            }
        }
        assert_eq!(dataset_i, params::params::TEST_DATA_SET_SIZE);

        let mut cv_features = [[0.0f32; params::dataset::N_FEATURES as usize]; params::params::FOLD_SIZE];
        let mut cv_classes = [false; params::params::FOLD_SIZE];

        dataset_i = 0;

        match self.partitions {
            None => panic!("Error, data not partitioned!"),
            Some(ref partitions) => {
                for (partition_i, partition) in partitions.iter().enumerate() {
                    if partition_i != trial_n {continue;}

                    for sample_i in partition.cases.iter() {
                        cv_features[dataset_i] = self.features[*sample_i];
                        cv_classes[dataset_i] = self.classes[*sample_i];
                        dataset_i += 1;
                    }

                    for sample_i in partition.controls.iter() {
                        cv_features[dataset_i] = self.features[*sample_i];
                        cv_classes[dataset_i] = self.classes[*sample_i];
                        dataset_i += 1;
                    }

                }
            }
        }
        assert_eq!(dataset_i, params::params::FOLD_SIZE);

        PartitionedDataSet{
            test: TestDataSet {features:test_features, classes:test_classes},
            cv: ValidationSet{features:cv_features, classes:cv_classes},
        }
    }


    pub fn get_parts(&self, trial_n: usize) -> (TestDataSet, ValidationSet) {
        let mut test_features = [[0.0f32; params::dataset::N_FEATURES as usize]; params::params::TEST_DATA_SET_SIZE];
        let mut test_classes = [false; params::params::TEST_DATA_SET_SIZE];

        let mut dataset_i = 0;

        match self.partitions {
            None => panic!("Error, data not partitioned!"),
            Some(ref partitions) => {
                for (partition_i, partition) in partitions.iter().enumerate() {
                    if partition_i == trial_n {continue;}

                    for sample_i in partition.cases.iter() {
                        test_features[dataset_i] = self.features[*sample_i];
                        test_classes[dataset_i] = self.classes[*sample_i];
                        dataset_i += 1;
                    }
                    for sample_i in partition.controls.iter() {
                        test_features[dataset_i] = self.features[*sample_i];
                        test_classes[dataset_i] = self.classes[*sample_i];
                        dataset_i += 1;
                    }
                }
            }
        }
        assert_eq!(dataset_i, params::params::TEST_DATA_SET_SIZE);

        let mut cv_features = [[0.0f32; params::dataset::N_FEATURES as usize]; params::params::FOLD_SIZE];
        let mut cv_classes = [false; params::params::FOLD_SIZE];

        dataset_i = 0;

        match self.partitions {
            None => panic!("Error, data not partitioned!"),
            Some(ref partitions) => {
                for (partition_i, partition) in partitions.iter().enumerate() {
                    if partition_i != trial_n {continue;}

                    for sample_i in partition.cases.iter() {
                        cv_features[dataset_i] = self.features[*sample_i];
                        cv_classes[dataset_i] = self.classes[*sample_i];
                        dataset_i += 1;
                    }

                    for sample_i in partition.controls.iter() {
                        cv_features[dataset_i] = self.features[*sample_i];
                        cv_classes[dataset_i] = self.classes[*sample_i];
                        dataset_i += 1;
                    }

                }
            }
        }
        assert_eq!(dataset_i, params::params::FOLD_SIZE);
        (TestDataSet {features:test_features, classes:test_classes},ValidationSet{features:cv_features, classes:cv_classes},)
    }




    // trial_n specifies which partition is used for cv
    pub fn get_dataset(&self, trial_n: usize) -> TestDataSet {
        let mut features = [[0.0f32; params::dataset::N_FEATURES as usize]; params::params::TEST_DATA_SET_SIZE];
        let mut classes = [false; params::params::TEST_DATA_SET_SIZE];

        let mut dataset_i = 0;

        match self.partitions {
            None => panic!("Error, data not partitioned!"),
            Some(ref partitions) => {
                for (partition_i, partition) in partitions.iter().enumerate() {
                    if partition_i == trial_n {continue;}

                    for sample_i in partition.cases.iter() {
                        features[dataset_i] = self.features[*sample_i];
                        classes[dataset_i] = self.classes[*sample_i];
                        dataset_i += 1;
                    }

                    for sample_i in partition.controls.iter() {
                        features[dataset_i] = self.features[*sample_i];
                        classes[dataset_i] = self.classes[*sample_i];
                        dataset_i += 1;
                    }
                }
            }
        }

        assert_eq!(dataset_i, params::params::TEST_DATA_SET_SIZE);
        TestDataSet {features, classes}
    }


    // consumes self, trial_n specifies which partition is used for cv
    pub fn to_test_dataset(self, trial_n: usize) -> TestDataSet {
        let mut features = [[0.0f32; params::dataset::N_FEATURES as usize]; params::params::TEST_DATA_SET_SIZE];
        let mut classes = [false; params::params::TEST_DATA_SET_SIZE];

        let mut dataset_i = 0;

        match self.partitions {
            None => panic!("Error, data not partitioned!"),
            Some(ref partitions) => {
                for (partition_i, partition) in partitions.iter().enumerate() {
                    if partition_i == trial_n {continue;}

                    for sample_i in partition.cases.iter() {
                        features[dataset_i] = self.features[*sample_i];
                        classes[dataset_i] = self.classes[*sample_i];
                        dataset_i += 1;
                    }
                    for sample_i in partition.controls.iter() {
                        features[dataset_i] = self.features[*sample_i];
                        classes[dataset_i] = self.classes[*sample_i];
                        dataset_i += 1;
                    }
                }
            }
        }
        assert_eq!(dataset_i, params::params::TEST_DATA_SET_SIZE);
        TestDataSet {features, classes}
    }


    pub fn get_cv_set(&self, trial_n: usize) -> ValidationSet {
        let mut features = [[0.0f32; params::dataset::N_FEATURES as usize]; params::params::FOLD_SIZE];
        let mut classes = [false; params::params::FOLD_SIZE];

        let mut dataset_i = 0;

        match self.partitions {
            None => panic!("Error, data not partitioned!"),
            Some(ref partitions) => {
                for (partition_i, partition) in partitions.iter().enumerate() {
                    if partition_i != trial_n {continue;}

                    for sample_i in partition.cases.iter() {
                        features[dataset_i] = self.features[*sample_i];
                        classes[dataset_i] = self.classes[*sample_i];
                        dataset_i += 1;
                    }

                    for sample_i in partition.controls.iter() {
                        features[dataset_i] = self.features[*sample_i];
                        classes[dataset_i] = self.classes[*sample_i];
                        dataset_i += 1;
                    }
                }
            }
        }
        assert_eq!(dataset_i, params::params::FOLD_SIZE);
        ValidationSet{features, classes}
    }


    pub fn to_cv_set(self, trial_n: usize) -> ValidationSet {
        let mut features = [[0.0f32; params::dataset::N_FEATURES as usize]; params::params::FOLD_SIZE];
        let mut classes = [false; params::params::FOLD_SIZE];

        let mut dataset_i = 0;

        match self.partitions {
            None => panic!("Error, data not partitioned!"),
            Some(ref partitions) => {
                for (partition_i, partition) in partitions.iter().enumerate() {
                    if partition_i != trial_n {continue;}

                    for sample_i in partition.cases.iter() {
                        features[dataset_i] = self.features[*sample_i];
                        classes[dataset_i] = self.classes[*sample_i];
                        dataset_i += 1;
                    }

                    for sample_i in partition.controls.iter() {
                        features[dataset_i] = self.features[*sample_i];
                        classes[dataset_i] = self.classes[*sample_i];
                        dataset_i += 1;
                    }

                }
            }
        }

        assert_eq!(dataset_i, params::params::FOLD_SIZE);
        ValidationSet{features, classes}
    }


    pub fn new(data_file: &str) -> FullDataSet{
        let mut features = [[0.0f32; params::dataset::N_FEATURES as usize]; params::dataset::N_SAMPLES];
        let mut classes = [false; params::dataset::N_SAMPLES];

        let f = File::open(data_file).unwrap();
        let mut csv_rdr = csv::Reader::from_reader(f);

        let mut count = 0;
        for (i,result) in csv_rdr.records().enumerate() {
            if let Ok(result) = result{
                let mut result_iter = result.iter();

                result_iter.next(); //skip first 2
                result_iter.next();

                match result_iter.next().unwrap() {
                    "0" => classes[i] = false,
                    "1" => classes[i] = true,
                    _ => panic!("Invalid classification field!!")
                }

                for (j, next_entry) in result_iter.enumerate() {
                    match next_entry.parse::<f32>() {
                        Ok(entry) => features[i][j] = entry,
                        Err(_) => features[i][j] = params::params::NA_TOKEN
                    }
                }
            }
        }

        let mut ds = FullDataSet{
            features, classes, partitions:None,
        };
        ds.set_partitions();
        ds
    }


    pub fn new_with_partitions(data_file: &str, partitions: &Vec<Partition>) -> FullDataSet{
        let mut features = [[0.0f32; params::dataset::N_FEATURES as usize]; params::dataset::N_SAMPLES];
        let mut classes = [false; params::dataset::N_SAMPLES];

        let f = File::open(data_file).unwrap();
        let mut csv_rdr = csv::Reader::from_reader(f);

        let mut count = 0;
        for (i,result) in csv_rdr.records().enumerate() {
            if let Ok(result) = result{
                let mut result_iter = result.iter();

                result_iter.next(); //skip first 2
                result_iter.next();

                match result_iter.next().unwrap() {
                    "0" => classes[i] = false,
                    "1" => classes[i] = true,
                    _ => panic!("Invalid classification field!!")
                }

                for (j, next_entry) in result_iter.enumerate() {
                    match next_entry.parse::<f32>() {
                        Ok(entry) => features[i][j] = entry,
                        Err(_) => features[i][j] = params::params::NA_TOKEN
                    }
                }

            }
        }
        FullDataSet{
            features, classes, partitions: Some(partitions.to_vec()),
        }
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
    if n >= params::params::POS_SAMPLE_RNG.start && n < params::params::POS_SAMPLE_RNG.end {true}
    else if n >= params::params::NEG_SAMPLE_RNG.start && n < params::params::NEG_SAMPLE_RNG.end {false}
    else {panic!("outside data range! {}", n)}
}

#[derive(Clone)]
pub struct Partition{
    cases: Vec<usize>,
    controls: Vec<usize>,
}