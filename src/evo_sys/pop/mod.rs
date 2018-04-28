pub mod maps;


use dataMgmt::logger::GenoEval;
use std::fs::File;
use std::io::Write;

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