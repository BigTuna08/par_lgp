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
use dataMgmt::logger::Logger;


pub trait Population {

    fn is_finished(&self) -> bool;
    fn can_send(&self) -> bool;

    fn next_new_prog(&mut self) -> Program; //mut so sent count is incremented
    fn try_put(&mut self, new_entry: EvalResult);

    fn update_cv(&mut self);

    fn log_full(&self, logger: &mut Logger); //Continuous logging.
    fn write_pop_info(&self, file_name: &str, eval: PopEval); //end of fold log
    fn write_genos(&self, file_name: &str); //end of fold log

//    fn get_sent_count(&self) -> u64;
//    fn get_recieved_count(&self) -> u64;
//    fn incr_sent(&mut self);
//    fn incr_recieved(&mut self);
}
//
//
//pub trait PopMap: Population{
//    fn get_config(&self) -> &PopConfig;
//
//    fn select_cell(&self, prog: &Program) -> (usize, usize);
//    fn compare_program(&self, new_prog: &Program, old_prog: &Program) -> bool;
//    fn is_in_bounds(&self, inds: &(usize,usize))-> bool;
//    fn get(&self, inds:  &(usize,usize)) -> &Option<Program>;
//    fn put(&mut self,  prog: Program, inds:  &(usize,usize));
//}




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