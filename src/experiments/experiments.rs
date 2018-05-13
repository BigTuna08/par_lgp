use params;
use dataMgmt::{DataSetManager, TestDataSet, ValidationSet};
use dataMgmt::Logger;
use dataMgmt::Message;
use evo_sys::{ResultMap, GenPop};
use threading::threadpool::ThreadPool;
//use experiments::FiveFoldMultiTrial;
use CoreConfig;
use PopInfo;

use std::fs::File;
use std::io::Write;
use std::fs::create_dir_all;
use std::sync::Arc;


pub fn multi_trial_five_fold_tracking(config: CoreConfig){

//    let root_out_dir = format!("{}/s{}_c{}", config.out_folder, config.select_cell_method, config.compare_prog_method);

    match create_dir_all(&config.out_folder) {
        Ok(_) =>{
            File::create(format!("{}/README.txt", &config.out_folder))
                .unwrap().write(format!("{:?}", &config).as_bytes());

            let mut logger = Logger::new(params::defaults::LOG_FREQ, &config.out_folder);
            logger.full_tracking();

            for _ in 0..config.n_iterations{
                five_fold_cv_tracking_ref(&mut logger, &config);
            }
        }
        Err(e) => panic!("Problem creating out dir! {:?}\n Err is {:?}", &config.out_folder, e)
    }
}

//
//pub fn five_fold_cv_tracking(logger: &mut Logger, config: &CoreConfig) {
//
//    //manages the data set by creating partitions, and shifting them after each fold
//    let mut data_manager = DataSetManager::new_rand_partition();
//
//    while let Some((test_data, cv_data)) = data_manager.next_set(){ //run 5 times
//        run_single_fold_tracking(test_data, cv_data, config, logger);
//    }
//}


pub fn five_fold_cv_tracking_ref(logger: &mut Logger, config: &CoreConfig) {

    //manages the data set by creating partitions, and shifting them after each fold
    let mut data_manager = DataSetManager::new_rand_partition(config.data_file.clone());

    println!("before trying to get refs");
    while let Some((test_data, cv_data)) = data_manager.next_set_refs(){ //run 5 times
        run_single_fold_tracking(test_data, cv_data, config, logger);
    }
}

fn run_single_fold_tracking(test_data: Arc<TestDataSet>, cv_data: Box<ValidationSet>, config: &CoreConfig, logger: &mut Logger) {
    println!("running fold");
    match config.pop_config {
        PopInfo::Gen(_) => {
            let mut res_map = GenPop::new(config.create_gen_pop_config(), cv_data);
            res_map.run_all_tracking(test_data, logger);
            logger.finish_fold_pop(res_map);
        },
        PopInfo::Map(_) => {
            let mut pop = ResultMap::new(config.create_result_map_config(), cv_data);
            pop.run_all_tracking(test_data, logger);
            logger.finish_fold(pop);
        },
    }
}

