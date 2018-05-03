pub mod maps;
pub mod selectors;
pub mod comparers;

use GenoEval;
use std::fs::File;
use std::io::Write;
use dataMgmt::message::EvalResult;
use evo_sys::prog::prog::Program;
use dataMgmt::dataset::ValidationSet;
use experiments::config::PopConfig;


pub trait Population {
//    fn new(config: PopConfig, cv_data: ValidationSet) -> Population;
    fn try_put(&mut self, new_entry: EvalResult);
    fn get_simple_mutated_genome_rand(&self) -> Program;

    fn update_cv(&mut self);
    fn get_pop_stats(&self, eval: PopEval) -> PopStats;

    fn write_pop_info(&self, file_name: &str, eval: PopEval);
    fn write_genos(&self, file_name: &str);
}

// new() -> Self
// is_finished() -> bool
// can_send()  -> bool
// new_prog() -> Program

// try_put(Program)
// log_full(Logger)

// get_config() -> &MapConfig
// get_cell_select() -> u8
// get_compare() -> u8

// get_sent_count -> u64
// get_received_count -> u64
// incr_sent() {sent_count++}
// incr_rece() {re++}

pub trait PopMap: Population{
//    fn try_put(&mut self, new_entry: EvalResult) {
//        self.recieved_count += 1;
//        let prog = new_entry.prog;
//        let inds = self.get_loc(&prog);
//        let mut replace = false;
//
//        if self.is_in_bounds(&inds){
//            match self.prog_map[inds.0][inds.1] {
//                Some(ref old_prog) => {
//                    if self.is_better(&prog, old_prog){
//                        replace = true
//                    }
//                }
//                None => replace = true
//            }
//        }
//
//        if replace {
//            self.put(prog, &inds)
//        }
//
//    }
//
//
//    //pick random prog from map and return mutated copy
//    fn get_simple_mutated_genome_rand(&self) -> Program {
//        let mut tries = 0;
//        let mut tr  = rand::thread_rng();
//
//        while tries < params::params::MAP_COLS*params::params::MAP_ROWS * 1000 {
//            if let Some(ref parent) = self.prog_map[tr.gen_range(0, params::params::MAP_ROWS)][tr.gen_range(0, params::params::MAP_COLS)] {
//                let prog = parent.test_mutate_copy();
//                let inds = self.get_loc(&prog);
//
//                if self.is_in_bounds(&inds){
//                    return prog
//                }
//            }
//        }
//        panic!("Timed out when trying to select a parent genome from results map!!");
//    }
//
//
//    fn update_cv(&mut self) {
//        for row_i in 0.. params::params::MAP_ROWS{
//            for col_i in 0.. params::params::MAP_COLS{
//                if let Some(ref mut genome) = self.prog_map[row_i][ col_i] {
//                    match genome.cv_fit {
//                        Some(_) => (),
//                        None => genome.cv_fit = Some(evo_sys::prog::eval::eval_program_cv(&genome, &self.cv_data)),
//                    }
//                }
//            }
//        }
//    }
//
//
//    fn get_pop_stats(&self, eval: PopEval) -> PopStats {
//        let mut best = std::f32::MIN;
//        let mut worst = std::f32::MAX;
//        let mut ave = 0.0f64;
//        let mut count = 0.0;
//
//        for row_i in 0.. params::params::MAP_ROWS{
//            for col_i in 0.. params::params::MAP_COLS{
//
//                if let Some(ref prog) = self.prog_map[row_i][ col_i]{
//                    let value = match eval {
//                        PopEval::TestFit => prog.test_fit.unwrap(),
//                        PopEval::CV => prog.cv_fit.unwrap(),
//                        PopEval::Geno(eval) => eval(prog),
//                    };
//
//                    ave += value as f64;
//                    count += 1.0;
//                    if value > best {best=value;}
//                    if value < worst {worst=value;}
//                }
//
//            }
//        }
//        ave = ave/count;
//
//        let mut vari = 0.0;
//        for row_i in 0.. params::params::MAP_ROWS{
//            for col_i in 0.. params::params::MAP_COLS{
//
//                if let Some(ref prog) = self.prog_map[row_i][ col_i]{
//                    let value = match eval {
//                        PopEval::TestFit => prog.test_fit.unwrap(),
//                        PopEval::CV => prog.cv_fit.unwrap(),
//                        PopEval::Geno(eval) => eval(prog),
//                    };
//                    vari += (value as f64-ave).powi(2);
//                }
//
//            }
//        }
//        vari /= count;
//        PopStats {best, worst, ave, sd:vari.sqrt()}
//    }
//
//
//    fn write_pop_info(&self, file_name: &str, eval: PopEval) {
//        let mut f = File::create(file_name).unwrap();
//
//        for row_i in 0..params::params::MAP_ROWS {
//            for col_i in 0..params::params::MAP_COLS {
//
//
//                let value = if let Some(ref prog) = self.prog_map[row_i][ col_i]{
//                    match eval {
//                        PopEval::TestFit => prog.test_fit.unwrap(),
//                        PopEval::CV => prog.cv_fit.unwrap(),
//                        PopEval::Geno(eval) => eval(prog),
//                    }
//
//                }else {
//                    params::params::MIN_FIT
//                };
//
//                f.write(value.to_string().as_bytes());
//                f.write(b"\t");
//            }
//            f.write(b"\n");
//        }
//    }
//
//
//    fn write_genos(&self, file_name: &str) {
//        let mut f = File::create(file_name).unwrap();
//        for row_i in 0..params::params::MAP_ROWS {
//            for col_i in 0..params::params::MAP_COLS {
//                if let Some(ref genome) = self.prog_map[row_i][ col_i] {
//                    f.write(b"(");
//                    f.write(row_i.to_string().as_bytes());
//                    f.write(b",");
//                    f.write(col_i.to_string().as_bytes());
//                    f.write(b")");
//                    f.write(b"\n");
//                    genome.write_effective_self_words(&mut f);
//                }
//            }
//        }
//    }
}


pub struct PopStats {
    pub best: f32,
    pub worst: f32,
    pub ave: f64,
    pub sd: f64,
//    pub count: f32,
}


impl PopStats {
    pub fn to_string(&self)->String{
        format!("best: {}\nworst: {}\nave: {}", self.best, self.worst, self.ave)
    }

    pub fn write_update(&self, f: &mut File, n_evals: u64){
        f.write(b"*after ");
        f.write(n_evals.to_string().as_bytes());
        f.write(b" evaluations\n");
        f.write(self.to_string().as_bytes());
        f.write(b"\n\n");
        f.flush();
    }
}


pub enum PopEval<'a>{
    TestFit,
    CV,
    Geno(&'a GenoEval),
}

//
//pub enum PutResult{
//    Failed,
//    Equal,
//    Improvement
//}