use dataMgmt::dataset::ValidationSet;
use dataMgmt::message::EvalResult;
use evo_sys;
use evo_sys::prog::prog::Program;
use params;
use rand;
use rand::Rng;
use std;
use std::fs::File;
use std::io::Write;
use super::{PopStats, PopEval, Population};
//use dataMgmt::message::Message;
use experiments::config::Config;
use experiments::config::{MapConfig};
use dataMgmt::logger::Logger;




pub struct ResultMap{
    prog_map: [[Option<Program>; params::params::MAP_COLS]; params::params::MAP_ROWS],
    pub config: MapConfig,
    cv_data: ValidationSet,
    sent_count: u64,
    pub recieved_count: u64,
}


impl<'a> Population for ResultMap {

    fn try_put(&mut self, new_entry: EvalResult) {
        self.recieved_count += 1;
        let prog = new_entry.prog;
        let inds = self.get_loc(&prog);
        let mut replace = false;

        if self.is_in_bounds(&inds){
            match self.prog_map[inds.0][inds.1] {
                Some(ref old_prog) => {
                    if self.is_better(&prog, old_prog){
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


    //pick random prog from map and return mutated copy
    fn get_simple_mutated_genome_rand(&self) -> Program {
        let mut tries = 0;
        let mut tr  = rand::thread_rng();

        while tries < params::params::MAP_COLS*params::params::MAP_ROWS * 1000 {
            if let Some(ref parent) = self.prog_map[tr.gen_range(0, params::params::MAP_ROWS)][tr.gen_range(0, params::params::MAP_COLS)] {
                return parent.test_mutate_copy()
            }
        }
        panic!("Timed out when trying to select a parent genome from results map!!");
    }


    fn update_cv(&mut self) {
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


    fn get_pop_stats(&self, eval: PopEval) -> PopStats {
        let mut best = std::f32::MIN;
        let mut worst = std::f32::MAX;
        let mut ave = 0.0f64;
        let mut count = 0.0;

        for row_i in 0.. params::params::MAP_ROWS{
            for col_i in 0.. params::params::MAP_COLS{

                if let Some(ref prog) = self.prog_map[row_i][ col_i]{
                    let value = match eval {
                        PopEval::TestFit => prog.test_fit.unwrap(),
                        PopEval::CV => prog.cv_fit.unwrap(),
                        PopEval::Geno(eval) => eval(prog),
                    };

                    ave += value as f64;
                    count += 1.0;
                    if value > best {best=value;}
                    if value < worst {worst=value;}
                }

            }
        }
        ave = ave/count;

        let mut vari = 0.0;
        for row_i in 0.. params::params::MAP_ROWS{
            for col_i in 0.. params::params::MAP_COLS{

                if let Some(ref prog) = self.prog_map[row_i][ col_i]{
                    let value = match eval {
                        PopEval::TestFit => prog.test_fit.unwrap(),
                        PopEval::CV => prog.cv_fit.unwrap(),
                        PopEval::Geno(eval) => eval(prog),
                    };
                    vari += (value as f64-ave).powi(2);
                }

            }
        }
        vari /= count;
        PopStats {best, worst, ave, sd:vari.sqrt(), count: count as f32}
    }


    fn write_pop_info(&self, file_name: &str, eval: PopEval) {
        let mut f = File::create(file_name).unwrap();

        for row_i in 0..params::params::MAP_ROWS {
            for col_i in 0..params::params::MAP_COLS {


                let value = if let Some(ref prog) = self.prog_map[row_i][ col_i]{
                    match eval {
                        PopEval::TestFit => prog.test_fit.unwrap(),
                        PopEval::CV => prog.cv_fit.unwrap(),
                        PopEval::Geno(eval) => eval(prog),
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


    fn write_genos(&self, file_name: &str) {
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
                    genome.write_effective_self_words(&mut f);
                }
            }
        }
    }
}

impl ResultMap{

    //hacked together method to log updates faster than previous method, which iterated over the map
    //many times. Currently the Map and logger class are too intertwined, and should be better organized
    pub fn log_full(&self, logger: &mut Logger){
        let mut count = 0.0;

        let n_evals = logger.geno_functions.len();
        let mut bests = vec![std::f32::MIN; n_evals+2];  // +2 for 2 fitnesses
        let mut worsts = vec![std::f32::MAX; n_evals+2];
        let mut aves = vec![0f64; n_evals+2];
        let mut varis = vec![0f64; n_evals+2]; //variences

        let mut feats_distr = [0; params::dataset::N_FEATURES as usize];


        for row_i in 0.. params::params::MAP_ROWS{
            for col_i in 0.. params::params::MAP_COLS{

                if let Some(ref prog) = self.prog_map[row_i][ col_i]{

                    for feat in prog.get_effective_feats(0) {
                        feats_distr[feat as usize] += 1;
                    }

                    let values = vec![prog.test_fit.unwrap(), prog.cv_fit.unwrap()];
                    let others: Vec<f32> = logger.geno_functions.iter().map(|f| f(prog)).collect();

                    for (i, value) in values.iter().chain(others.iter()).enumerate(){
                        aves[i] += *value as f64;
                        count += 1.0;
                        if *value > bests[i] {bests[i] = *value}
                        if *value < worsts[i] {worsts[i] =*value }
                    }
                }
            }
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

        logger.log_test_fits(PopStats{
            best:bests[0],
            worst:worsts[0],
            ave:aves[0],
            sd:varis[0].sqrt(),
            count,
        });

        logger.log_cv_fits(PopStats{
            best:bests[1],
            worst:worsts[1],
            ave:aves[1],
            sd:varis[1].sqrt(),
            count,
        });

        for i in 0..n_evals{
            logger.log_geno_stat(PopStats{
                best:bests[i+2],
                worst:worsts[i+2],
                ave:aves[i+2],
                sd:varis[i+2].sqrt(),
                count,
            }, i);
        }

        let unique_feat_count = feats_distr.iter().fold(0u8, |mut acc, x| {if *x > 0 {acc+=1;} acc});
        logger.log_feat_count(unique_feat_count);
        logger.log_feat_distr(&feats_distr);
    }


    pub fn count_eff_feats(&self) -> u8 {
        let mut feats = [false; params::dataset::N_FEATURES as usize];
        let mut count = 0;
        for row_i in 0..params::params::MAP_ROWS {
            for col_i in 0..params::params::MAP_COLS {
                if let Some(ref genome) = self.prog_map[row_i][ col_i] {
                    for feat in genome.get_effective_feats(0) {
                        if !feats[feat as usize] {
                            feats[feat as usize] = true;
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }


    pub fn eff_feats_distr(&self) -> [u8; params::dataset::N_FEATURES as usize] {
        let mut feats_distr = [0; params::dataset::N_FEATURES as usize];
        for row_i in 0..params::params::MAP_ROWS {
            for col_i in 0..params::params::MAP_COLS {
                if let Some(ref genome) = self.prog_map[row_i][ col_i] {
                    for feat in genome.get_effective_feats(0) {
                        feats_distr[feat as usize] += 1;
                    }
                }
            }
        }
        feats_distr
    }
}


impl<'a> ResultMap {

    pub fn is_finished(&self) -> bool{
        self.recieved_count >= self.config.total_evals
    }


    pub fn get_new_prog(&mut self) -> Program{
        self.sent_count += 1;
        if self.sent_count <= self.config.initial_pop as u64{
            Program::new_default_range()
        }
        else {
            self.get_simple_mutated_genome_rand()
        }
    }


    pub fn pending_evals(&self)-> u64{
        self.sent_count - self.recieved_count
    }

    pub fn is_empty(&self)-> bool{
        self.recieved_count == 0
    }


    pub fn can_send(&self)->bool{
        if self.pending_evals() >= params::params::THREAD_POOL_MAX{
            return false;
        }
        (self.recieved_count > 0) || (self.sent_count < self.config.initial_pop as u64)
    }


    fn get_test_fit(&self, inds: &(usize, usize)) -> f32 {
        match self.prog_map[inds.0][inds.1] {
            Some(ref prog) => prog.test_fit.unwrap(),
            None => params::params::MIN_FIT,
        }
    }

    fn get_cv_fit(&self, inds: &(usize, usize)) -> f32 {
        match self.prog_map[inds.0][inds.1] {
            Some(ref prog) => prog.cv_fit.unwrap(),
            None => params::params::MIN_FIT,
        }
    }

    fn put(&mut self, val: Program, inds: &(usize, usize)) {
        self.prog_map[inds.0][inds.1] = Some(val);
    }

    fn is_in_bounds(&self, inds: &(usize, usize))-> bool{
        (inds.0 < params::params::MAP_ROWS) && (inds.1 < params::params::MAP_COLS)
    }

}


impl<'a> ResultMap {
    pub fn new( config: MapConfig, cv_data: ValidationSet) -> ResultMap {
        ResultMap {
            prog_map:
            [[None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None], ],
            config,
            sent_count: 0,
            recieved_count: 0,
            cv_data,
        }
    }
}
