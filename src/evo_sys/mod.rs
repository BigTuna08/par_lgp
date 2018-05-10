pub mod prog;
pub mod pop;
pub mod params;

use params as global_params;
use dataMgmt::{ValidationSet, Logger, TestDataSet};
use GenoEval;
use ResultMapConfig;
use GenPopConfig;



////      Program structs   ////

#[derive(Debug)]
pub struct Program{
    pub n_calc_regs: u8,
    pub features: Vec<u8>,
    pub instructions: Vec<Instruction>,
    pub test_fit: Option<f32>,
    pub cv_fit: Option<f32>,
}


#[derive(Copy, Clone, Debug)]
pub struct Instruction{
    pub dest: u8,
    pub op: u8,
    pub src1: u8,
    pub src2: u8,
}


////      Population structs   ////

pub trait Runnable{
    fn run_all(&mut self, test_data: TestDataSet);
    fn run_all_tracking(&mut self, test_data: TestDataSet, logger: &mut Logger);
}

pub struct ResultMap{
    prog_map: [[Option<Program>; global_params::params::MAP_COLS]; global_params::params::MAP_ROWS],
    pub config: ResultMapConfig,
    cv_data: Box<ValidationSet>,
    sent_count: u64,
    pub recieved_count: u64,
}


pub struct GenPop{
    progs: Vec<Program>,
    config: GenPopConfig,
    cv_data: Box<ValidationSet>,
    current_gen: u32,
    current_gen_recived: usize,
    current_gen_sent: usize,
}

//
//#[derive(Debug)]
//pub struct PopConfig {
//    pub select_cell_method: u8,
//    pub compare_prog_method: u8,
//    pub initial_pop: u32,
//    pub total_evals: u64,
//}



////      Other   ////

pub enum ProgInspectRequest<'a>{
    TestFit,
    CV,
    Geno(&'a GenoEval),
}
