use progSystem::pop::maps3::{MapStats, ResultMap};
use progSystem::prog::Program;
use params;
use std::fs::create_dir;
use std::fs::File;
use std::io::Write;
use dataMgmt::trackers;
use progSystem::pop::maps3::PopEval;

pub type GenoEval = Fn(&Program) -> f32 + 'static;




struct FileSet{
    best: File,
    worst: File,
    ave: File,
    sd: File,
}

impl FileSet{
    fn new(output_dir: &str) -> FileSet{
        let best = File::create(format!("{}/best.txt", output_dir)).unwrap();
        let worst = File::create(format!("{}/worst.txt", output_dir)).unwrap();
        let ave = File::create(format!("{}/ave.txt", output_dir)).unwrap();
        let sd = File::create(format!("{}/sd.txt", output_dir)).unwrap();
        FileSet{best, worst, ave, sd}
    }

    pub fn write(&mut self, stat: MapStats){
        self.best.write(stat.best.to_string().as_bytes());
        self.worst.write(stat.worst.to_string().as_bytes());
        self.ave.write(stat.ave.to_string().as_bytes());
        self.sd.write(stat.sd.to_string().as_bytes());

        self.best.write(b"\t");
        self.worst.write(b"\t");
        self.ave.write(b"\t");
        self.sd.write(b"\t");
    }

    pub fn write_new_line(&mut self){
        self.best.write(b"\n");
        self.worst.write(b"\n");
        self.ave.write(b"\n");
        self.sd.write(b"\n");
    }

    pub fn flush(&mut self){
        self.best.flush();
        self.worst.flush();
        self.ave.flush();
        self.sd.flush();
    }

}

pub struct Logger{
    pub freq: u32,
    pub root_dir: String,

    test_output_files: Option<FileSet>,
    cv_output_files: Option<FileSet>,
    geno_output_files: Vec<FileSet>,

    pub geno_functions: Vec<&'static GenoEval>,

    current_iter: u16,
    current_fold: u8, //assumes 5 fold
}


impl Logger{
    pub fn new(freq: u32, root_dir: &str) -> Logger {
        create_dir(format!("{}/genos", root_dir));
        create_dir(format!("{}/cv_fit_maps", root_dir));
        create_dir(format!("{}/test_fit_maps", root_dir));

        Logger{
            freq,
            root_dir: String::from(root_dir),
            test_output_files: None,
            cv_output_files: None,
            geno_output_files: Vec::new(),
            geno_functions: Vec::new(),
            current_iter:0,
            current_fold: 0,
        }
    }

    //assumes full tracking !!
    pub fn update(&mut self, res_map: &ResultMap){

        self.log_test_fits(res_map.get_map_stats(PopEval::TestFit));
        self.log_cv_fits(res_map.get_map_stats(PopEval::CV));

        for i in 0..self.geno_functions.len(){
            let stats = res_map.get_map_stats(PopEval::Geno(&self.geno_functions[i]));
            self.log_geno_stat(stats,i);
        }
    }


    pub fn finish_fold(&mut self, final_results: ResultMap){

        final_results.write_genos(
            &format!("{}/genos/iter{}-fold{}.txt", self.root_dir, self.current_iter, self.current_fold));
        final_results.write_cv_fits(
            &format!("{}/cv_fit_maps/iter{}-fold{}.txt", self.root_dir, self.current_iter, self.current_fold));
        final_results.write_test_fits(
            &format!("{}/test_fit_maps/iter{}-fold{}.txt", self.root_dir, self.current_iter, self.current_fold));

        self.new_line();
        self.update_fold_iter();
    }


    fn update_fold_iter(&mut self){
        if self.current_fold + 1 < params::N_FOLDS{
            self.current_fold += 1;
        }
        else {
            self.current_fold =0;
            self.current_iter += 1;
            self.flush();
        }
    }

}


// for tracking
impl Logger{

    pub fn full_tracking(&mut self){
        self.track_both_fits();

        self.add_geno_tracker("abs_len", &trackers::get_abs_geno_len);
        self.add_geno_tracker("eff_len", &trackers::get_eff_geno_len);
        self.add_geno_tracker("eff_feats", &trackers::get_eff_feats);
    }

    pub fn track_both_fits(&mut self) {
        self.track_test();
        self.track_cv();
    }


    pub fn track_test(&mut self){
        match self.test_output_files {
            Some(_) => panic!("Already tracking test!!!"),
            None => {
                let test_dir = format!("{}/test_fits", self.root_dir);
                create_dir(&test_dir);
                self.test_output_files = Some(FileSet::new(&test_dir));
            },
        }
    }

    pub fn track_cv(&mut self){
        match self.cv_output_files {
            Some(_) => panic!("Already tracking cv!!!"),
            None => {
                let cv_dir = format!("{}/cv_fits", self.root_dir);
                create_dir(&cv_dir);
                self.cv_output_files = Some(FileSet::new(&cv_dir));
            },
        }
    }

    pub fn add_geno_tracker(&mut self, name: &str, geno_eval: &'static GenoEval){
        let out_dir = format!("{}/{}", self.root_dir, name);
        create_dir(&out_dir);
        self.geno_output_files.push(FileSet::new(&out_dir));
        self.geno_functions.push(geno_eval);
    }
}


// for writing
impl Logger{
    pub fn log_test_fits(&mut self, stats: MapStats){
        match self.test_output_files {
            Some(ref mut out_f) => out_f.write(stats),
            None => panic!("No test out file!!")
        };
    }

    pub fn log_cv_fits(&mut self, stats: MapStats){
        match self.cv_output_files {
            Some(ref mut out_f) => out_f.write(stats),
            None => panic!("No cv out file!!")
        };
    }

    pub fn log_geno_stat(&mut self, stats: MapStats, stat_ind: usize){
        self.geno_output_files[stat_ind].write(stats);
    }


    pub fn new_line(&mut self){
        match self.test_output_files {
            Some(ref mut out_f) => out_f.write_new_line(),
            None => (),
        };
        match self.cv_output_files {
            Some(ref mut out_f) => out_f.write_new_line(),
            None => (),
        };
        for f in self.geno_output_files.iter_mut(){
            f.write_new_line();
        }
    }


    pub fn flush(&mut self){
        match self.test_output_files {
            Some(ref mut out_f) => out_f.flush(),
            None => (),
        };
        match self.cv_output_files {
            Some(ref mut out_f) => out_f.flush(),
            None => (),
        };
        for f in self.geno_output_files.iter_mut(){
            f.flush();
        }
    }

}