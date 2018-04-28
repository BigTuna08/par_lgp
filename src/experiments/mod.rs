use config::Config;
use params;
use threading::threadpool::ThreadPool;
use dataMgmt::dataset::{FullDataSet, TestDataSet, ValidationSet, DataSetManager};
use dataMgmt::logger::Logger;
use progSystem::pop::maps3::{MapStats, PutResult, ResultMap};
use dataMgmt::message::EvalResult;
use progSystem::prog::Program;
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
use dataMgmt;



pub fn multi_trial_five_fold_tracking(mut config: Config){

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


pub fn five_fold_cv_tracking(logger: &mut Logger, config: &Config) {

    //manages the data set by creating partitions, and shifting them after each fold
    let mut data_manager = DataSetManager::new_rand_partition();

    while let Some((test_data, cv_data)) = data_manager.next_set(){ //run 5 times
        run_single_fold_tracking(test_data, cv_data, config, logger);
    }
}




fn run_single_fold_tracking(test_data: TestDataSet, cv_data: ValidationSet, config: &Config, logger: &mut Logger) {
    let mut sent_count: u64 = 0;
    let mut recieved_count: u64 = 0;
    let mut res_map = ResultMap::new();
    let mut pool = ThreadPool::new(params::N_THREADS, test_data, config.get_current_eval_code());

    while sent_count < config.initial_pop as u64{  //initilize pop: Programs are randomly created
        if sent_count - recieved_count < params::THREAD_POOL_MAX {
            pool.add_task(EvalResult::new(Program::new_default_range()));
            sent_count += 1;
        }
        else {
            res_map.try_put_trial_based_config(pool.next_result_wait(), recieved_count, &config, 1);
            recieved_count += 1;
        }

        if sent_count % logger.freq as u64 == 0 && recieved_count > 0{  // update log
            res_map.update_cross_validation(&cv_data);
            logger.update(&res_map);
        }

    }


    while recieved_count < config.total_evals { //continue until finished: new programs are offspring of old
        if (sent_count - recieved_count < params::THREAD_POOL_MAX) && (recieved_count > 0) {
            pool.add_task(EvalResult::new(res_map.get_simple_mutated_genome_rand()));
            sent_count += 1;
        }
        else {
            res_map.try_put(pool.next_result_wait());
            recieved_count += 1;
        }

        if sent_count % logger.freq as u64 == 0 && recieved_count > 0{ // update log
            res_map.update_cross_validation(&cv_data);
            logger.update(&res_map);
        }
    }
    pool.terminate();
    logger.finish_fold(res_map);
}


