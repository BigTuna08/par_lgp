pub mod config;

//use config::Config;
use self::config::Config;
use self::config::FiveFoldMultiTrial;
use self::config::MapConfig;
use dataMgmt;
use dataMgmt::dataset::{DataSetManager, FullDataSet, TestDataSet, ValidationSet};
use dataMgmt::logger::Logger;
use dataMgmt::message::EvalResult;
use evo_sys::pop::maps::{ResultMap};
use evo_sys::pop::{PopStats, PutResult};
use evo_sys::prog::prog::Program;
use params;
use rand::{seq, thread_rng};
use rand::distributions::Range;
use rand::distributions::Sample;
use rand::Rng;
use std::fs::create_dir;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use threading::threadpool::ThreadPool;
use evo_sys::pop::Population;



pub fn multi_trial_five_fold_tracking(mut config: FiveFoldMultiTrial){

    let root_out_dir = format!("results/{}/raw", config.out_folder);
    match create_dir_all(&root_out_dir) {
        Ok(_) =>{
            File::create(format!("{}/README.txt", &root_out_dir))
                .unwrap().write(format!("{:?}", &config).as_bytes());

            let mut logger = Logger::new(10_000, &root_out_dir);
            logger.full_tracking();

            for _ in 0..config.n_iter{
                five_fold_cv_tracking(&mut logger, &config);
            }
        }
        Err(e) => panic!("Problem creating out dir! {:?}\n Err is {:?}", &root_out_dir, e)
    }
}


pub fn five_fold_cv_tracking(logger: &mut Logger, config: &FiveFoldMultiTrial) {

    //manages the data set by creating partitions, and shifting them after each fold
    let mut data_manager = DataSetManager::new_rand_partition();

    while let Some((test_data, cv_data)) = data_manager.next_set(){ //run 5 times
        run_single_fold_tracking(test_data, cv_data, config, logger);
    }
}



fn run_single_fold_tracking(test_data: TestDataSet, cv_data: ValidationSet, config: &FiveFoldMultiTrial, logger: &mut Logger) {
    let mut res_map = ResultMap::new(config.get_map_config());
    let mut pool = ThreadPool::new(params::N_THREADS, test_data, 17);  //fix here!! no 17!


    while !res_map.is_finished() {
        if res_map.can_send() {
            pool.add_task(EvalResult::new(res_map.get_new_prog()))
        }
        else {
            res_map.try_put(pool.next_result_wait());
        }

        if sent_count % logger.freq as u64 == 0 && recieved_count > 0{  // update log
            res_map.update_cv(&cv_data);
            logger.update(&res_map);
        }
    }


    pool.terminate();
    res_map.update_cv(&cv_data);
    logger.finish_fold(res_map);
}
