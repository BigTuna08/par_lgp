use params::params as global_params;
use super::super::{GenPop, Program, params, Runnable};
use dataMgmt::{Message, EvalResult, ValidationSet, TestDataSet};
use threading::threadpool::ThreadPool;
use dataMgmt;
use evo_sys::ProgInspectRequest;
use evo_sys;
use dataMgmt::Logger;
use evo_sys::pop::PopStats;
use GenPopConfig;

use std::io::Write;
use std::fs::File;
use std::collections::HashSet;
use rand::thread_rng;
use rand::Rng;
use std::f32;
use std::cmp::Ordering;
use std::sync::Arc;

impl GenPop{
    pub fn run_all(&mut self, test_data: TestDataSet){

    }
    pub fn run_all_tracking(&mut self, test_data: Arc<TestDataSet>, logger: &mut Logger){
        let mut pool = ThreadPool::new(global_params::N_THREADS, test_data);
        let t_size = self.config.tourn_size as usize;
        let mut_method = self.config.mutate_method;
        self.initialize(&mut pool);
        while !self.is_finished() { //do a generation
            while !self.sent_all() {
                pool.add_task(Message::Cont(self.get_mutated_genome_tournament(t_size, mut_method)));   //nonmeta!
            }
            while !self.recieved_all() {
                self.try_put(pool.next_result_wait());
            }
            self.next_gen();
            self.update_cv();
            self.log_full(logger);
            assert_eq!(pool.current_job_count(), 0);
        }

        pool.terminate();
        self.update_cv();
    }
}


impl GenPop{

    pub fn new(config: GenPopConfig, cv_data: Box<ValidationSet>)->GenPop{
        GenPop{
            progs: Vec::with_capacity(2*params::GEN_POP_SIZE),
            cv_data,
            config,
            current_gen:0,
            current_gen_recived: 0,
            current_gen_sent: 0,
        }
    }

    pub fn initialize(&mut self, thread_pool: &mut ThreadPool){
        for _ in 0..params::GEN_POP_SIZE{
            thread_pool.add_task(Message::Cont(self.new_random_prog()));
        }

        for _ in 0..params::GEN_POP_SIZE*2{
            self.progs.push(Program::new_empty());
        }

        let mut recieved = 0;
        while recieved < params::GEN_POP_SIZE {
            self.progs[recieved] = thread_pool.next_result_wait().prog;
            recieved += 1;
        }
    }

    pub fn run_random_gens(&mut self, thread_pool: &mut ThreadPool, n_gens: u32){

        for _ in 0..n_gens{

            for _ in 0..params::GEN_POP_SIZE{
                thread_pool.add_task(Message::Cont(self.new_random_prog()));
            }


            let mut recieved = 0;
            while recieved < params::GEN_POP_SIZE {
                self.progs[params::GEN_POP_SIZE as usize + recieved] = thread_pool.next_result_wait().prog;
                recieved += 1;
            }
            self.next_gen();
        }


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


    pub fn get_mutated_genome_tournament(&mut self, t_size: usize, mutation_code: u8) -> Program{

        if t_size > params::GEN_POP_SIZE{panic!("Tournament is bigger than pop! Cannot select enough");}
        if self.current_gen_sent >= params::GEN_POP_SIZE{panic!("Trying to send to many progs for a gen!");}
        self.current_gen_sent += 1;

        // select progams by index
        let mut choosen = HashSet::with_capacity(t_size);
        let mut best_fit = f32::MIN;
        let mut rng = thread_rng();
        while choosen.len() < t_size {
            choosen.insert(rng.gen_range(0, params::GEN_POP_SIZE));
        }

        // choose best of selected
        let mut best_prog = None;
        for i in choosen {
            let fit = self.progs[i].test_fit.unwrap();
            if fit > best_fit || (fit == best_fit && rng.gen()){// if tie 50/50 random
                best_prog = Some(&self.progs[i]);
                best_fit = fit;
            }
        }
        best_prog.unwrap().mutate_copy(mutation_code)
    }


    pub fn try_put(&mut self, new_entry: EvalResult){
        if self.current_gen_recived >= params::GEN_POP_SIZE {
            panic!("trying to add to many programs for a generation!\n call next_gen()!")
        }
        self.progs[self.current_gen_recived] = new_entry.prog;
        self.current_gen_recived += 1;
    }

    pub fn next_gen(&mut self){
        self.current_gen += 1;
        self.current_gen_sent = 0;
        self.current_gen_recived = 0;
        self.progs.sort_unstable_by(|a,b| simple_tie_rand(a,b))
    }

    pub fn recieved_all(&self)->bool {
        self.current_gen_recived == params::GEN_POP_SIZE
    }

    pub fn sent_all(&self)->bool {
        self.current_gen_sent == params::GEN_POP_SIZE
    }

    pub fn is_finished(&self)-> bool{
        self.current_gen == self.config.total_gens
    }
}


impl GenPop{
    //hacked together method to log updates faster than previous method, which iterated over the map
    //many times. Currently the Map and logger class are too intertwined, and should be better organized
    pub fn log_full(&self, logger: &mut Logger){
        let mut count = 0.0;

        let n_evals = logger.geno_functions.len();
        let mut bests = vec![f32::MIN; n_evals+2];  // +2 for 2 fitnesses
        let mut worsts = vec![f32::MAX; n_evals+2];
        let mut aves = vec![0f64; n_evals+2];
        let mut varis = vec![0f64; n_evals+2]; //variences

        let mut feats_distr = [0; dataMgmt::params::N_FEATURES as usize];


        for prog in self.progs.iter() {
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

        for value in aves.iter_mut(){
            *value /= count;
        }

        for prog in self.progs.iter() {
            let values = vec![prog.test_fit.unwrap(), prog.cv_fit.unwrap()];
            let others: Vec<f32> = logger.geno_functions.iter().map(|f| f(prog)).collect();
            for (i, value) in values.iter().chain(others.iter()).enumerate(){
                varis[i] += (*value as f64-aves[i]).powi(2);
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

        for prog in self.progs.iter() {
            let value = match eval {
                ProgInspectRequest::TestFit => prog.test_fit.unwrap(),
                ProgInspectRequest::CV => prog.cv_fit.unwrap(),
                ProgInspectRequest::Geno(eval) => eval(prog),
            };
            f.write(value.to_string().as_bytes());
            f.write(b"\t");
        }
    }


    pub fn write_genos(&self, file_name: &str) {
        let mut f = File::create(file_name).unwrap();
        for prog in self.progs.iter() {
            prog.write_effective_self_words(&mut f);
        }
    }

    pub fn update_cv(&mut self) {
        for prog in self.progs.iter_mut() {
            match prog.cv_fit {
                Some(_) => (),
                None => prog.cv_fit = Some(evo_sys::prog::eval::eval_program_cv(&prog, &self.cv_data)),
            }
        }
    }
}


fn simple_tie_rand(new_prog: &Program, old_prog: &Program) -> Ordering{
    if new_prog.test_fit.unwrap() == old_prog.test_fit.unwrap(){
        if thread_rng().gen(){ return Ordering::Greater}
        else { return Ordering::Less }
    }
    else {
        return new_prog.test_fit.unwrap().partial_cmp(&old_prog.test_fit.unwrap()).unwrap()
    }
}


