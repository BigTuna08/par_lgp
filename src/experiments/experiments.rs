use params;
use dataMgmt::dataset::{DataSetManager, TestDataSet, ValidationSet};
use dataMgmt::logger::Logger;
use dataMgmt::message::Message;
use evo_sys::pop::maps::{ResultMap};
use threading::threadpool::ThreadPool;
use experiments::config::FiveFoldMultiTrial;
use evo_sys::pop::Population;

use std::fs::File;
use std::io::Write;
use std::fs::create_dir_all;



pub fn multi_trial_five_fold_tracking(mut config: FiveFoldMultiTrial){

    let root_out_dir = format!("{}/s{}_c{}", config.out_folder, config.select_cell_method, config.compare_prog_method);

    match create_dir_all(&root_out_dir) {
        Ok(_) =>{
            File::create(format!("{}/README.txt", &root_out_dir))
                .unwrap().write(format!("{:?}", &config).as_bytes());

            let mut logger = Logger::new(params::defaults::LOG_FREQ, &root_out_dir);
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
    let mut res_map = ResultMap::new(config.get_map_config(), cv_data);
    let mut pool = ThreadPool::new(params::params::N_THREADS, test_data, 17);  //fix here!! no 17!

    while !res_map.is_finished() {
        if res_map.can_send() {
            pool.add_task(Message::Cont(res_map.get_new_prog()))
        }
            else {
                res_map.try_put(pool.next_result_wait());
                if res_map.recieved_count % logger.freq as u64 == 0 {
                    res_map.update_cv();
                    logger.update(&res_map);
                }
            }
    }

    pool.terminate();
    res_map.update_cv();
    logger.finish_fold(res_map);
}