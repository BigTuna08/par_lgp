use csv;
use csv::ReaderBuilder;
use params as global_params;
use rand;
use rand::Rng;
use std::fs::File;
use std::sync::Arc;

use super::{DataRecord, FullDataSet, params, Partition, DataSetManager, TestDataSet, ValidationSet};


impl DataRecord{
    fn new_blank()->DataRecord{
        DataRecord{
            features: [0.0; params::N_FEATURES as usize],
            class: false,
        }
    }
}

impl FullDataSet{

    pub fn new(data_file: &str) -> Box<FullDataSet>{
//        println!("Begin getting new full data set");

        let mut records =  Box::new([DataRecord::new_blank(); params::N_SAMPLES]);
//        println!("records init");
        let f = File::open(data_file).unwrap();

        let mut csv_rdr = ReaderBuilder::new()
            .delimiter(b'\t')
            .from_reader(f);

//        println!("Geeting new data! {:?}", csv_rdr.headers().unwrap().len());

//        let mut csv_rdr = csv::Reader::from_reader(f);
//        println!("before iter");
        for (i,result) in csv_rdr.records().enumerate() {
            if let Ok(result) = result{
//                println!("result is {:?}", result);
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
                        Err(e) => {
//                            features[j] = global_params::params::NA_TOKEN
//                            print!("Error reading something!! i={} j={} err is {:?}", i, j, e);
                            panic!("error getting inputs!, change code if dataset containt missing");
                        }
                    }
                }
                records[i] = DataRecord{features, class};
            }
            else {
                panic!("bad record! i={}, {:?}", i, &result);
            }
        }
//        println!("Before returning");


        Box::new(FullDataSet{
            records:*records,
        })
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

        while cases.len() < params::N_POS_FOLD || controls.len() < params::N_NEG_FOLD {
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

            if tries >= global_params::params::DUPLICATE_TIME_OUT*2{
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


impl DataSetManager{

//    pub fn new(partitions: Vec<Partition>) -> DataSetManager{
//        DataSetManager{partitions, current_partition:0}
//    }

    pub fn new_rand_partition(data_file: String) -> DataSetManager{
        DataSetManager{partitions:gen_partitions(), current_partition:0, data_file}
    }

    pub fn next_set_refs(&mut self) -> Option<(Arc<TestDataSet>, Box<ValidationSet>)>{

//        println!("getting refs!");
        if self.current_partition >= params::N_FOLDS{return None}
//        println!("in next_Set before trying to load");

        let mut test_records = &mut [DataRecord::new_blank(); params::TEST_DATA_SET_SIZE];
//        println!("got test");
        let mut cv_records = &mut [DataRecord::new_blank(); params::FOLD_SIZE];

        let mut test_dataset_i = 0;
        let mut cv_dataset_i = 0;
//        println!("got cv");
        let full_set = FullDataSet::new(&self.data_file);

//        println!("after getting full set");

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
        Some((Arc::new(TestDataSet { records:*test_records,}),
              Box::new(ValidationSet{ records:*cv_records,})))
    }

    pub fn next_set(&mut self) -> Option<(TestDataSet, ValidationSet)>{

        if self.current_partition >= params::N_FOLDS{return None}

        let mut test_records = [DataRecord::new_blank(); params::TEST_DATA_SET_SIZE];
        let mut cv_records = [DataRecord::new_blank(); params::FOLD_SIZE];

        let mut test_dataset_i = 0;
        let mut cv_dataset_i = 0;

        let full_set = FullDataSet::new(&self.data_file);

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

