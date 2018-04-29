pub mod maps;


use dataMgmt::logger::GenoEval;
use std::fs::File;
use std::io::Write;
use dataMgmt::message::EvalResult;
use evo_sys::prog::prog::Program;
use dataMgmt::dataset::ValidationSet;

pub trait Population {
    fn try_put(&mut self, new_entry: EvalResult) -> PutResult;
    fn get_simple_mutated_genome_rand(&self) -> Program;

    fn update_cv(&mut self, data: &ValidationSet);
    fn get_pop_stats(&self, eval: PopEval) -> PopStats;

    fn write_pop_info(&self, file_name: &str, eval: PopEval);
    fn write_genos(&self, file_name: &str);
}


pub struct PopStats {
    pub best: f32,
    pub worst: f32,
    pub ave: f64,
    pub sd: f64,
    pub count: f32,
}


impl PopStats {
    pub fn to_string(&self)->String{
        format!("best: {}\nworst: {}\nave: {}\ncount: {}", self.best, self.worst, self.ave, self.count)
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


pub enum PutResult{
    Failed,
    Equal,
    Improvement
}