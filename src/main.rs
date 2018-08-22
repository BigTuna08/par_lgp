extern crate csv;
extern crate parLGP;
extern crate rand;
extern crate serde;
extern crate time;



use std::env;

use std::fs::create_dir;
use time::PreciseTime;
use parLGP::Runner;
use parLGP::dataAnal::ValueType;

use std::collections::HashMap;


//    parLGP::dataMgmt::metabolites::print_headers("inputs/new.txt");
//    parLGP::evo_sys::prog::registers::make_regs();
fn main() {


    let start = PreciseTime::now();
    println!("SINGLE THREAD FOR DEBUG");


    let mut args: Vec<String> = env::args().collect();
    println!("ARGS {:?}", args);


//    println!("metabolite i is {}", parLGP::dataMgmt::metabolites::get_metabolite_index("Arg"));

    let data = parLGP::dataMgmt::FullDataSet::new("inputs/data3.csv");


    let m1 = ValueType::Metabolite("total_DMA".to_string());
    let m2 =  ValueType::Metabolite("Ser".to_string());

    println!("ratio= {}", parLGP::dataAnal::get_comparison_ratio(m1,m2, &*data) );
//    println!("case: {:?} < {:?} ratio= {}", m1, m2, parLGP::dataAnal::get_conditional_comparison_ratio(m1,m2, &*data, true) );
//    println!("not : {:?} < {:?} ratio= {}", m1, m2, parLGP::dataAnal::get_conditional_comparison_ratio(m1,m2, &*data, false) );
//
//    println!("ratio= {}", parLGP::dataAnal::get_less_than_ratio("Arg", -0.333, &*data) );

//    let mut runner = Runner::new("configs/experiment.txt");
//    println!("runner {:?}", runner);
//    runner.run_all_configs();


    let end = PreciseTime::now();
    println!("{} seconds full program execution.", start.to(end));
}


fn test(){
    let x = 4.0;
    let y = 0.0f64;
    let z = x / y;
    let w = y.log(0.0);

    println!("{:?}", w > 0.0);
    println!("{:?}", w < 0.0);
    println!("{:?}", w == 0.0);
    println!("{:?}", w);
}



//fn comp_times(){
//    let i = 20_000;
//
//    let start2 = PreciseTime::now();
//    parLGP::test_a2s(&parLGP::dataMgmt::logger::a_2_s, i);
//    let end2 = PreciseTime::now();
//    let t2 = start2.to(end2);
//
//    let start = PreciseTime::now();
//    parLGP::test_a2s(&parLGP::dataMgmt::logger::array_2_str, i);
//    let end = PreciseTime::now();
//    let t1 = start.to(end);
//
//
//
//
//
//    println!("{} for 1\t{} for 2", t1, t2);
//}







//
//fn var_pen_test_big(){
//
//
//
//    let total_evals = 10_000_000;
//    let initial_pop = 250_000;
//    let n_iter = 5;
//
//    let comment = "testing varible placement penalties: 25M eval with 2M period penalty from 6-9M and 16-19M";
//    let root_folder = String::from("pens2");
//
//    for i in 10..15{
//        let out_folder = format!("{}/method{}", root_folder, i);
//        let config = Config{initial_pop, total_evals, out_folder, n_iter, comment:String::from(comment), current_eval_code_i:None, eval_codes:vec![0,5]};
//        parLGP::multi_trial_five_fold_tracking_pen(config, i);
//    }
//
//
//}
//
//fn var_pen_test(){
//    let total_evals = 25_000_000;
//    let initial_pop = 250_000;
//    let n_iter = 5;
//
//    let comment = "testing varible placement penalties: 25M eval with 2M period penalty from 6-9M and 16-19M";
//    let out_folder = String::from("pens1");
//
//    let config = Config{initial_pop, total_evals, out_folder, n_iter, comment:String::from(comment), current_eval_code_i:None, eval_codes:vec![0,5]};
//    parLGP::multi_trial_five_fold_tracking(config);
//
//}
//
//fn compare_initial_sizes(){
//    let max_evals = vec![250_000, 1_000_000, 5_000_000, 10_000_000];
//    let initail_pops = vec![1_000, 10_000, 50_000, 250_000, 1_000_000];
//    let n_iter = 5;
//
//    let comment = "comparison of max_evals X initial_pop";
//    let root_folder = "compare";
//
//    for max_eval in max_evals.iter(){
//        for initial_pop_size in initail_pops.iter(){
//            let start = PreciseTime::now();
//            let out_folder = format!("{}/max_eval{}/initial_pop{}", root_folder, max_eval, initial_pop_size);
//
//            let config = Config{initial_pop: *initial_pop_size, total_evals: *max_eval, out_folder, n_iter, comment:String::from(comment), current_eval_code_i:None, eval_codes:vec![0]};
//            parLGP::multi_trial_five_fold_tracking(config);
//            let end = PreciseTime::now();
//            println!("{} seconds for this condition. ({} in minutes)", start.to(end), start.to(end).num_minutes());
//        }
//    }
//}

//
//fn test(){
//    for i in 0..60{
//        println!("{}-{}",i, parLGP::evo_sys::prog::eval::get_mapped_index(i));
//    }
////    let i = 60;
////    println!("{}-{}",i, parLGP::evo_sys::prog::eval::get_mapped_index(i));
//}

//fn multi(big_config: Config){
//    create_dir(format!("results/{}", big_config.out_folder));
//    for (i, desc) in parLGP::evo_sys::prog::eval::EVALS_DESC.iter().enumerate(){
//        let config = Config{
//            initial_pop: big_config.initial_pop,
//            total_evals: big_config.total_evals,
//            out_folder: format!("{}/eval{}", big_config.out_folder, i),
//            n_iter: big_config.n_iter,
//            eval_code: i as usize,
//            eval_desc: desc.to_string(),
//            comment: big_config.comment.clone(),
//        };
//        parLGP::multi_trial_five_fold_tracking(config);
//    }
//}

