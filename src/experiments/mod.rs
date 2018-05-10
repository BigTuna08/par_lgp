pub mod experiments;
pub mod mgmt;

use std::fs::File;
use std::io::Write;

#[derive(Debug)]
pub struct FiveFoldMultiTrial{
    pub select_cell_method: u8,
    pub compare_prog_method: u8,
    pub initial_pop: u32,
    pub total_evals: u64,
    pub n_iter: u32,
    pub out_folder: String,
    pub comment: String,
}


#[derive(Debug)]
pub struct FiveFoldMultiTrialGenPop{
    pub pop_size: usize,
    pub tourn_size: usize,
    pub total_gens: u32,
    pub random_gens: u32,
    pub n_iter: u32,
    pub out_folder: String,
    pub comment: String,
}


#[derive(Debug)]
pub struct Manager{
    pub select_cell_methods: Vec<u8>,
    pub compare_prog_methods: Vec<u8>,
    pub initial_pop: u32,
    pub total_evals: u64,
    pub n_iter: u32,
    pub out_folder: String,
    pub comment: String,
}


#[derive(Debug)]
pub struct Manager2{
    pub methods: Vec<(u8,u8)>,
    pub initial_pop: u32,
    pub total_evals: u64,
    pub n_iter: u32,
    pub out_folder: String,
    pub comment: String,
}


#[derive(Debug)]
pub struct GroupPopManager{
    pub pop_sizes: Vec<usize>,
    pub tourn_sizes: Vec<usize>,
    pub random_gen_amounts: Vec<u32>,
    pub total_gens: u32,
    pub n_iter: u32,
    pub out_folder: String,
    pub comment: String,
}



pub trait ExperimentRunner{
    //in the future maybe make this a get_next_experiment_config -> FiveFoldConfig
    fn run_all(&self);
}


impl ExperimentRunner for Manager {
    fn run_all(&self) {

        File::create(format!("{}/README.txt", &self.out_folder))
            .unwrap().write(format!("{:?}", &self).as_bytes());

        for s in self.select_cell_methods.iter(){
            for c in self.compare_prog_methods.iter(){
                let config = FiveFoldMultiTrial{
                    select_cell_method:*s,
                    compare_prog_method:*c,
                    initial_pop: self.initial_pop,
                    total_evals: self.total_evals,
                    n_iter: self.n_iter,
                    comment: self.comment.clone(),
                    out_folder: self.out_folder.clone(),
                };
//                experiments::multi_trial_five_fold_tracking(config);
            }
        }
    }
}



impl ExperimentRunner for Manager2 {
    fn run_all(&self) {

        File::create(format!("{}/README.txt", &self.out_folder))
            .unwrap().write(format!("{:?}", &self).as_bytes());

        for &(s,c) in self.methods.iter(){
            let config = FiveFoldMultiTrial{
                select_cell_method:s,
                compare_prog_method:c,
                initial_pop: self.initial_pop,
                total_evals: self.total_evals,
                n_iter: self.n_iter,
                comment: self.comment.clone(),
                out_folder: self.out_folder.clone(),
            };
//            experiments::multi_trial_five_fold_tracking(config);
        }
    }
}