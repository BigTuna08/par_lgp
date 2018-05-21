use dataMgmt::ValidationSet;
use dataMgmt::{ Message, EvalResult, TestDataSet};
use evo_sys;

use params;
use rand;
use rand::Rng;
use std;
use std::fs::File;
use std::io::Write;
use super::super::{Program, ProgInspectRequest, ResultMap};
use super::{PopStats};
use threading::threadpool::ThreadPool;

use dataMgmt::Logger;
use dataMgmt;
use ResultMapConfig;
use std::sync::Arc;

impl ResultMap{
    pub fn run_all(&mut self, test_data: TestDataSet){

    }
    pub fn run_all_tracking(&mut self, test_data: Arc<TestDataSet>, logger: &mut Logger){
        let mut pool = ThreadPool::new_default( test_data);
        let mutate_method = self.config.mutate_method;

        while !self.is_finished() {
            if pool.can_recieve() {
                pool.add_task(Message::Cont(self.next_new_prog(mutate_method)));
            }
                else {
                    self.try_put(pool.next_result_wait());
                    if self.recieved_count % logger.freq as u64 == 0 {
                        self.update_cv();
                        self.log_full(logger);
                    }
                }
        }

        pool.terminate();
        self.update_cv();
    }
}


impl ResultMap {

    pub fn is_finished(&self) -> bool{
        self.recieved_count >= self.config.n_evals
    }


    pub fn new_random_prog(&self) -> Program{
        Program::new_random_range(self.config.prog_defaults.INITIAL_INSTR_MIN,
                                  self.config.prog_defaults.INITIAL_INSTR_MIN,
                                  self.config.prog_defaults.INITIAL_CALC_REG_MIN,
                                  self.config.prog_defaults.INITIAL_CALC_REG_MAX,
                                  self.config.prog_defaults.INITIAL_N_OPS_MIN,
                                  self.config.prog_defaults.INITIAL_N_OPS_MAX,
                                  self.config.prog_defaults.INITIAL_FEAT_MIN,
                                  self.config.prog_defaults.INITIAL_FEAT_MAX,)
    }


    pub fn next_new_prog(&mut self, mutation_code: u8) -> Program{
        self.sent_count += 1;
        if self.sent_count <= self.config.initial_pop as u64{
            let mut prog = self.new_random_prog();
            let mut tries = 0;
            while !self.is_in_bounds(&self.select_cell(&prog)) && tries < params::params::DUPLICATE_TIME_OUT{
                prog = self.new_random_prog();
                tries += 1;
            }
//            println!("** about to start mutating existing, coverage is {}, sent count is {}", self.get_percent_covered(), self.sent_count);
            prog
        }
        else {
            self.get_simple_mutated_genome_rand(mutation_code)
        }
    }


    pub fn try_put(&mut self, new_entry: EvalResult) {
        self.recieved_count += 1;
        let prog = new_entry.prog;
        let inds = self.select_cell(&prog);
        let mut replace = false;

        if self.is_in_bounds(&inds){
            match self.prog_map[inds.0][inds.1] {
                Some(ref old_prog) => {
                    if self.compare(&prog, old_prog){
                        replace = true
                    }
                }
                None => replace = true
            }
        }

        if replace {
            self.put(prog, &inds)
        }

    }

//    pub fn can_send(&self)->bool{
//        if self.pending_evals() >= params::params::THREAD_POOL_MAX{
//            return false;
//        }
//        (self.recieved_count > 0) || (self.sent_count < self.config.initial_pop as u64)
//    }



    //hacked together method to log updates faster than previous method, which iterated over the map
    //many times. Currently the Map and logger class are too intertwined, and should be better organized
    pub fn log_full(&self, logger: &mut Logger){
        let mut count = 0.0;

        let n_evals = logger.geno_functions.len();
        let mut bests = vec![std::f32::MIN; n_evals+2];  // +2 for 2 fitnesses
        let mut worsts = vec![std::f32::MAX; n_evals+2];
        let mut aves = vec![0f64; n_evals+2];
        let mut varis = vec![0f64; n_evals+2]; //variences

        let mut feats_distr = [0; dataMgmt::params::N_FEATURES as usize];


        for row_i in 0.. params::params::MAP_ROWS{
            for col_i in 0.. params::params::MAP_COLS{

                if let Some(ref prog) = self.prog_map[row_i][ col_i]{
                    count += 1.0;
                    for feat in prog.get_effective_feats(0) {
                        feats_distr[feat as usize] += 1;
                    }

                    let values = vec![prog.test_fit.unwrap(), prog.cv_fit.unwrap()];
                    let others: Vec<f32> = logger.geno_functions.iter().map(|f| f(prog)).collect();

                    for (i, value) in values.iter().chain(others.iter()).enumerate(){
                        aves[i] += *value as f64;

                        if *value > bests[i] {bests[i] = *value}
                        if *value < worsts[i] {worsts[i] =*value }
                    }

                }
            }
        }

        for value in aves.iter_mut(){
            *value /= count;
        }

        for row_i in 0.. params::params::MAP_ROWS{
            for col_i in 0.. params::params::MAP_COLS{

                if let Some(ref prog) = self.prog_map[row_i][ col_i]{

                    let values = vec![prog.test_fit.unwrap(), prog.cv_fit.unwrap()];
                    let others: Vec<f32> = logger.geno_functions.iter().map(|f| f(prog)).collect();

                    for (i, value) in values.iter().chain(others.iter()).enumerate(){
                        varis[i] += (*value as f64-aves[i]).powi(2);
                    }
                }
            }
        }

        for value in varis.iter_mut(){
            *value /= count;
        }

        logger.log_test_fits(PopStats{
            best:bests[0],
            worst:worsts[0],
            ave:aves[0],
            sd:varis[0].sqrt(),
        });

        logger.log_cv_fits(PopStats{
            best:bests[1],
            worst:worsts[1],
            ave:aves[1],
            sd:varis[1].sqrt(),
        });

        for i in 0..n_evals{
            logger.log_geno_stat(PopStats{
                best:bests[i+2],
                worst:worsts[i+2],
                ave:aves[i+2],
                sd:varis[i+2].sqrt(),
            }, i);
        }

        let unique_feat_count = feats_distr.iter().fold(0u8, |mut acc, x| {if *x > 0 {acc+=1;} acc});
        logger.log_feat_count(unique_feat_count);
        logger.log_feat_distr(&feats_distr);
    }


    pub fn write_pop_info(&self, file_name: &str, eval: ProgInspectRequest) {
        let mut f = File::create(file_name).unwrap();


        for row_i in 0..params::params::MAP_ROWS {
            for col_i in 0..params::params::MAP_COLS {


                let value = if let Some(ref prog) = self.prog_map[row_i][ col_i]{
                    match eval {
                        ProgInspectRequest::TestFit => prog.test_fit.unwrap(),
                        ProgInspectRequest::CV => prog.cv_fit.unwrap(),
                        ProgInspectRequest::Geno(eval) => eval(prog),
                    }

                }else {
                    params::params::MIN_FIT
                };

                f.write(value.to_string().as_bytes());
                f.write(b"\t");
            }
            f.write(b"\n");
        }
    }


    pub fn write_genos(&self, file_name: &str) {
        let mut f = File::create(file_name).unwrap();
        for row_i in 0..params::params::MAP_ROWS {
            for col_i in 0..params::params::MAP_COLS {
                if let Some(ref genome) = self.prog_map[row_i][ col_i] {
                    f.write(b"(");
                    f.write(row_i.to_string().as_bytes());
                    f.write(b",");
                    f.write(col_i.to_string().as_bytes());
                    f.write(b")");
                    f.write(b"\n");
                    genome.write_header(&mut f);
                    f.write(b"\n");
                    genome.write_self_words(&mut f);
                    f.write(b"\n");
                    genome.write_effective_self_words(&mut f);
                    f.write(b"\n");
                    f.write(b"\n");
                }
            }
        }
    }

    pub fn update_cv(&mut self) {
        for row_i in 0.. params::params::MAP_ROWS{
            for col_i in 0.. params::params::MAP_COLS{
                if let Some(ref mut genome) = self.prog_map[row_i][ col_i] {
                    match genome.cv_fit {
                        Some(_) => (),
                        None => genome.cv_fit = Some(evo_sys::prog::eval::eval_program_cv(&genome, &self.cv_data)),
                    }
                }
            }
        }
    }


    pub fn printout_pop_info(&self) {
        println!("\n** Pop Info **\n\n");
        for row_i in 0..params::params::MAP_ROWS {
            for col_i in 0..params::params::MAP_COLS {
                println!("{:?}", &self.prog_map[row_i][ col_i]);
            }
        }
    }

}


impl ResultMap {

    //pick random prog from map and return mutated copy
    fn get_simple_mutated_genome_rand(&self, mutation_code: u8) -> Program {
        let mut tries = 0;
        let mut tr  = rand::thread_rng();

        while tries < params::params::MAP_COLS*params::params::MAP_ROWS * 10000 {
            if let Some(ref parent) = self.prog_map[tr.gen_range(0, params::params::MAP_ROWS)][tr.gen_range(0, params::params::MAP_COLS)] {
                let prog = parent.mutate_copy(mutation_code);
                let inds = self.select_cell(&prog);

                if self.is_in_bounds(&inds){
                    tries = 0;
                    return prog
                }else  {
//                    println!("mutated to out of bounds! inds are {:?} after {} tries", &inds, tries);
                }
            }
            tries += 1;
        }
        //self.printout_pop_info();
        panic!("Timed out when trying to select a parent genome from results map!!");
    }


    fn pending_evals(&self)-> u64{
        self.sent_count - self.recieved_count
    }


    fn put(&mut self, val: Program, inds: &(usize, usize)) {
        self.prog_map[inds.0][inds.1] = Some(val);
    }

    fn is_in_bounds(&self, inds: &(usize, usize))-> bool{
        (inds.0 < params::params::MAP_ROWS) && (inds.1 < params::params::MAP_COLS)
    }

    fn get_percent_covered(&self)->f64{
        let mut total = 0.0;
        let mut exist = 0.0;
        for row_i in 0..params::params::MAP_ROWS {
            for col_i in 0..params::params::MAP_COLS {
                if let Some(_) = self.prog_map[row_i][ col_i] {
                    exist += 1.0;
                }
                total += 1.0;
            }
        }
        exist/total
    }

}


impl ResultMap {
    pub fn new(config: ResultMapConfig, cv_data: Box<ValidationSet>) -> ResultMap {
        ResultMap {
//            prog_map:
//            [[None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
//                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None], ],
            prog_map: [[None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
            ],
            config,
            sent_count: 0,
            recieved_count: 0,
            cv_data,
        }
    }
}
