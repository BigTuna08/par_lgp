extern crate csv;
extern crate indexmap;
extern crate rand;
extern crate serde;
extern crate time;
//
//use config::Config;
//use dataMgmt::dataset::{FullDataSet, TestDataSet, ValidationSet};
//use dataMgmt::logger::Logger;
//use progSystem::pop::maps::{MapStats, PutResult, ResultMap};
//use dataMgmt::message::EvalResult;
//use progSystem::prog::Program;
//use self::rand::{seq, thread_rng};
//use self::rand::distributions::Range;
//use self::rand::distributions::Sample;
//use self::rand::Rng;
//use std::fs::create_dir;
//use std::fs::create_dir_all;
//use std::fs::File;
//use std::io::Write;
//use std::sync::{Arc, Mutex};
//use std::sync::atomic::AtomicBool;
//use std::sync::atomic::Ordering;
//use std::sync::mpsc;
//use std::thread;
//use std::time::Duration;
//use time::PreciseTime;
//
////#![feature(associated_consts)]
pub mod params;
pub mod config;
pub mod progSystem;
pub mod threading;
pub mod dataMgmt;
pub mod experiments;


pub fn heads(){
    let h = dataMgmt::dataset::get_headers(params::DATA);
    print!("headers = [");
    for n in h.iter(){
        println!("\"{}\",",n);
    }
    print!("]");
}
//
//
////use progSystem::eval;
//
//
//pub const n: u32 = 10_000;
//
//pub fn multi_trial_five_fold_tracking(mut config: Config){
//    let root_out_dir = format!("results/{}/raw", config.out_folder);
//    match create_dir_all(&root_out_dir) {
//        Ok(_) =>{
//
//            File::create(format!("{}/README.txt", &root_out_dir))
//                .unwrap().write(format!("{:?}", &config).as_bytes());
//
//            while config.next_eval_code() {
//
//                let current_eval_dir = format!("{}/eval{}", &root_out_dir, config.get_current_eval_code());
//                println!("Starting with current eval dir {}", current_eval_dir);
//                match create_dir(&current_eval_dir) {
//                    Ok(_) => {
//                        let mut logger = Logger::new(10_000, &current_eval_dir);
//                        logger.track_both_fits();
//
//                        logger.add_geno_tracker("abs_len", &get_abs_geno_len);
//                        logger.add_geno_tracker("eff_len", &get_eff_geno_len);
//                        logger.add_geno_tracker("eff_feats", &get_eff_feats);
//
//                        for i in 0..config.n_iter{
//                            five_fold_cv_tracking(&mut logger, &config, i);
////                            logger.new_line();
//                            logger.flush(); //update regularly so can check , remove to speed up a little
//                        }
//                    },
//                    Err(e) => panic!("Error creating result dir for path {:?}\nerror is {:?}\n",&current_eval_dir, e),
//                }
//
//
//            }
//        }
//        Err(e) => panic!("Problem creating out dir! {:?}\n Err is {:?}", &root_out_dir, e)
//    }
//}
//
//
//
//pub fn multi_trial_five_fold_tracking_pen(mut config: Config, pen_method: u8){
//    let root_out_dir = format!("results/{}/raw", config.out_folder);
//    match create_dir_all(&root_out_dir) {
//        Ok(_) =>{
//
//            File::create(format!("{}/README.txt", &root_out_dir))
//                .unwrap().write(format!("{:?}", &config).as_bytes());
//
//            while config.next_eval_code() {
//
//                let current_eval_dir = format!("{}/eval{}", &root_out_dir, config.get_current_eval_code());
//                println!("Starting with current eval dir {}", current_eval_dir);
//                match create_dir(&current_eval_dir) {
//                    Ok(_) => {
//                        let mut logger = Logger::new(10_000, &current_eval_dir);
//                        logger.track_both_fits();
//
//                        logger.add_geno_tracker("abs_len", &get_abs_geno_len);
//                        logger.add_geno_tracker("eff_len", &get_eff_geno_len);
//                        logger.add_geno_tracker("eff_feats", &get_eff_feats);
//
//                        for i in 0..config.n_iter{
//                            five_fold_cv_tracking_pen(&mut logger, &config, i, pen_method);
////                            logger.new_line();
//                            logger.flush(); //update regularly so can check , remove to speed up a little
//                        }
//                    },
//                    Err(e) => panic!("Error creating result dir for path {:?}\nerror is {:?}\n",&current_eval_dir, e),
//                }
//
//
//            }
//        }
//        Err(e) => panic!("Problem creating out dir! {:?}\n Err is {:?}", &root_out_dir, e)
//    }
//}
//
//
////pub fn multi_trial_five_fold(config: Config){
////    let new_root_out_folder = format!("results/{}", config.out_folder);
////    match create_dir(&new_root_out_folder) {
////        Ok(_) =>{
////            {
////                let read_me_file = format!("{}/README.txt", &new_root_out_folder);
////                let contents = format!("{:?}", &config);
////                let mut read_me_file = File::create(read_me_file).unwrap();
////                read_me_file.write(contents.as_bytes());
////                read_me_file.write(b"\n");
////                let contents = format!("Initial pop: {}\nEvals: {}\n", config.initial_pop, config.total_evals);
////                read_me_file.write(contents.as_bytes());
////            }
////            for i in 0..config.n_iter{
////
////                let mut results_path = new_root_out_folder.clone();
////                results_path.push_str( format!("/trial{}",i).as_str()); // root/trial<i>
////
////                match create_dir(&results_path) {
////                    Ok(_) => five_fold_cv_tracking(&results_path, &config),
////                    Err(e) => println!("Error creating result dir for path {:?}\nerror is {:?}\n",&results_path, e),
////                }
////            }
////        }
////        Err(e) => panic!("Problem creating out dir! {:?}\n Err is {:?}", &new_root_out_folder, e)
////    }
////
////}
//
//pub fn five_fold_cv_tracking_pen(logger: &mut Logger, config: &Config, iter: u32, pen_method: u8){
//    let data_file = params::DATA;
//    let data = FullDataSet::new(data_file);
//
//    let headers = dataMgmt::dataset::get_headers(data_file);
//    let data_partitions = dataMgmt::dataset::gen_partitions();
//
//    create_dir(format!("{}/genos", logger.root_dir));
//    create_dir(format!("{}/cv_fit_maps", logger.root_dir));
//    create_dir(format!("{}/test_fit_maps", logger.root_dir));
//    for i in 0..5 {  //once each fold
//        let (test_data, cv_data) = data.get_parts(i);
//        let results =
//            run_single_fold_tracking_pen(test_data, cv_data, config, logger, pen_method);
//
//        results.write_genos(&format!("{}/genos/iter{}-fold{}.txt", logger.root_dir, iter, i), &headers);
//        results.write_cv_fits(&format!("{}/cv_fit_maps/iter{}-fold{}.txt", logger.root_dir, iter, i));
//        results.write_test_fits(&format!("{}/test_fit_maps/iter{}-fold{}.txt", logger.root_dir, iter, i));
//        logger.new_line();
//    }
//}
//
//
//pub fn five_fold_cv_tracking(logger: &mut Logger, config: &Config, iter: u32){
//    let data_file = params::DATA;
//    let data = FullDataSet::new(data_file);
//
//    let headers = dataMgmt::dataset::get_headers(data_file);
//    let data_partitions = dataMgmt::dataset::gen_partitions();
//
//    create_dir(format!("{}/genos", logger.root_dir));
//    create_dir(format!("{}/cv_fit_maps", logger.root_dir));
//    create_dir(format!("{}/test_fit_maps", logger.root_dir));
//    for i in 0..5 {  //once each fold
//        let (test_data, cv_data) = data.get_parts(i);
//        let results =
//            run_single_fold_tracking(test_data, cv_data, config, logger);
//
//        results.write_genos(&format!("{}/genos/iter{}-fold{}.txt", logger.root_dir, iter, i), &headers);
//        results.write_cv_fits(&format!("{}/cv_fit_maps/iter{}-fold{}.txt", logger.root_dir, iter, i));
//        results.write_test_fits(&format!("{}/test_fit_maps/iter{}-fold{}.txt", logger.root_dir, iter, i));
//        logger.new_line();
//    }
//}
//
//pub fn get_abs_geno_len(p: &Program) -> f32{
//    p.get_abs_len() as f32
//}
//
//pub fn get_eff_geno_len(p: &Program) -> f32{
//    p.get_effective_len(0) as f32
//}
//
//pub fn get_eff_feats(p: &Program) -> f32{
//    p.get_n_effective_feats(0) as f32
//}
//
//
//// !! Keep - faster bc keeps cv seperate
////pub fn five_fold_cv(results_path: &str, config: &Config){
//////    let results_path = "results/test0";
////    let data_file = params::DATA;
////
////    let headers = dataMgmt::dataset::get_headers(data_file);
////    let data_partitions = dataMgmt::dataset::gen_partitions();
////
////    let result_log_file = format!("{}/result_log.txt", results_path);
////    let mut result_log_file = File::create(result_log_file).unwrap();
////
////    let cont_log_file = format!("{}/cont_log.txt", results_path);
////    let mut cont_log_file = File::create(cont_log_file).unwrap();
////
////    for i in 0..5 {  //once each fold
////        cont_log_file.write(b"#Begining fold #");
////        cont_log_file.write(i.to_string().as_bytes());
////        cont_log_file.write(b"\n");
////
////
////        let start = PreciseTime::now();
////        let results = { //run on test data
////            let test_data = TestDataSet::get_dataset(i, &data_partitions, data_file);
////            run_single_fold(test_data, &config, &mut cont_log_file )
////        };
////
////
////        let mut time_str = start.to(PreciseTime::now()).num_minutes().to_string();
////        time_str.push_str(" mins");
////        log_prog_info(&results, i, results_path, &headers);
//////        log_result(&mut result_log_file, results.get_stats(), time_str, i, config.total_evals);
////
////
////        let cv_stats = { //run on cv data
////            let cv_data = ValidationSet::get_validation_dataset(i, &data_partitions, data_file);
////            run_cv(cv_data, results, i, results_path)
////        };
////        result_log_file.write(b"\nCross Validation Results:\n");
////        result_log_file.write(cv_stats.to_string().as_bytes());
////        result_log_file.write(b"\n\n\n");
////        result_log_file.flush();
////    }
////}
//
//
//fn log_prog_info(results: &ResultMap, trial_n: usize, results_folder: &str, feat_names: &Vec<String>){
//    let fit_fname = format!("{}/test{}-fits.txt", results_folder, trial_n);
//    results.write_test_fits(&fit_fname);
//
//    let geno_fname = format!("{}/run{}-genos.txt", results_folder, trial_n);
//    results.write_genos(&geno_fname, feat_names);
//
//    let abs_len_fname = format!("{}/run{}-abslen.txt", results_folder, trial_n);
//    results.write_genos_abs_len(&abs_len_fname);
//
//    let eff_len_fname = format!("{}/run{}-efflen.txt", results_folder, trial_n);
//    results.write_genos_eff_len(&eff_len_fname);
//}
//
//
//fn log_result(log_file: &mut File, map_res: MapStats, time_str: String, i: usize, evals: u64){
//    log_file.write(b"Finished iteration #");
//    log_file.write(i.to_string().as_bytes());
//    log_file.write(b" (");
//    log_file.write(evals.to_string().as_bytes());
//    log_file.write(b" evaluations in ");
//    log_file.write(time_str.as_bytes());
//    log_file.write(b")\n\nOn test data:\n");
//    log_file.write(map_res.to_string().as_bytes());
//    log_file.write(b"\n");
//}
//
//
//
//fn run_single_fold_tracking_pen(test_data: TestDataSet, cv_data: ValidationSet, config: &Config, logger: &mut Logger, pen_method: u8) -> ResultMap{
//    let mut pool = threading::threadpool::ThreadPool::new(params::N_THREADS, test_data, config.get_current_eval_code());
//    let mut pool_cap = 10000;
//
//    let mut sent_count: u64 = 0;
//    let mut recieved_count: u64 = 0;
//
//    let mut res_map = ResultMap::new();
//    let mut map_change = false;
//
//
//    //lazy, could make it stop sending after send count, but not it keeps sending until enough are recived
//    while recieved_count < config.total_evals {
//        let mut get_next = false;
//
//        match sent_count < config.initial_pop as u64{
//            true => {
//                if sent_count - recieved_count < pool_cap {
//                    pool.add_task(EvalResult::new(Program::new_default_range()));
//                    sent_count += 1;
//                } else { get_next = true;}
//            },
//
//            false => {
//                if (sent_count - recieved_count < pool_cap) && (recieved_count > 0) {
//                    pool.add_task(
//                        EvalResult::new(res_map.get_simple_mutated_genome_rand())
//                    );
//                    sent_count += 1;
//                } else { get_next = true;}
//            },
//        }
//
//        if get_next{
//            match res_map.try_put_trial_based_config(pool.next_result_wait(), recieved_count, &config, pen_method) {
//                PutResult::Improvement => map_change = true,
//                _ => (),
//            }
//            recieved_count += 1;
//        }
//
//        if sent_count % logger.freq as u64 == 0 && recieved_count > 0{
//            logger.log_test_fits(res_map.get_test_stats());
//            logger.log_cv_fits(res_map.update_cross_validation(&cv_data));
//            for i in 0..logger.geno_functions.len(){
//                let stats = res_map.get_geno_stats(logger.geno_functions[i]);
//                logger.log_geno_stat(stats,i);
//            }
//        }
//
//
//    }
//    pool.terminate();
//    res_map
//}
//
//
////
////fn run_single_fold(data: TestDataSet, config: &Config, cont_log_file: &mut File) -> ResultMap{
////    let mut pool = threading::threadpool::ThreadPool::new(params::N_THREADS, data, config.eval_code);
////    let mut pool_cap = 10000u32;
////
////    for _ in 0..pool_cap {
////        pool.add_task(Message::new(Program::new_default_range() ) );
////    }
////
////    let mut res_map = ResultMap::new();
////    let mut count: u64 = 0;
////    let mut output_n = 0;
////    let mut map_change = false;
////
////
////    while count < config.initial_pop as u64  {
////        match pool.next_result() {
////            Some(res) => {
////                match res_map.try_put(res) {
////                    PutResult::Improvement => map_change = true,
////                    _ => (),
////                }
////                pool.add_task(Message::new(Program::new_default_range()));
////                count += 1;
////            },
////            None => (),
////        }
////    }
////
////    while count < config.total_evals {
////        match pool.next_result() {
////            Some(res) => {
////                count += 1;
////
////                match res_map.try_put(res) {
////                    PutResult::Improvement => map_change = true,
////                    _ => (),
////                }
////
////                let new_geno = res_map.get_simple_mutated_genome_rand();
////                pool.add_task(Message::new(new_geno));
////
////                if count % 500_000 == 0 {
////                    let stats = res_map.update_cross_validation();
////                    println!("Count is now: {}, map: {:?}", count, &stats.to_string());
////                    stats.write_update(cont_log_file, count);
////
////                }
////            },
////            None => (),// println!("GOT NONE"),
////        }
////    }
////    pool.terminate();
////    res_map
////}
//
//
