pub mod dataset;
pub mod metabolites;  //string names of metabolites (headers in csv file)
pub mod trackers; //functions for tracking info about programs during evolution
pub mod logger;
pub mod params;


use GenoEval;
use evo_sys::Program;
use std::fs::File;
use std::slice::Iter;


////      Dataset structs   ////

#[derive(Copy, Clone)]
pub struct DataRecord{
    pub features: [f32; params::N_FEATURES as usize],
    pub class: bool,
}


pub trait DataSet{
    fn record_iter(&self) -> Iter<DataRecord>;
    fn size(&self) -> usize;
}


pub struct FullDataSet{
    pub records: [DataRecord; params::N_SAMPLES],
}


pub struct TestDataSet {
    pub records: [DataRecord; params::TEST_DATA_SET_SIZE],
}


pub struct ValidationSet{
    pub records: [DataRecord; params::FOLD_SIZE],
}


#[derive(Clone)]
pub struct Partition{
    cases: Vec<usize>,
    controls: Vec<usize>,
}


pub struct DataSetManager{
    partitions: Vec<Partition>,
    current_partition: u8,
    data_file: String,
}


////      Logger structs   ////

pub struct Logger{
    pub freq: u32,
    pub root_dir: String,

    test_output_files: Option<FileSet>,
    cv_output_files: Option<FileSet>,
    geno_output_files: Vec<FileSet>,

    pub geno_functions: Vec<&'static GenoEval>,

    feature_count: Option<File>,
    feature_distr: Option<File>,

    current_iter: u16,
    current_fold: u8, //assumes 5 fold
}

struct FileSet{
    max: File,
    min: File,
    ave: File,
    sd: File,
}

////      Other structs   ////

pub struct EvalResult{
    pub prog: Program
}


#[derive(Debug)]
pub enum Message {
    Cont(Program),
    Quit,
}



impl EvalResult{
    pub fn new(prog: Program)-> EvalResult{
        EvalResult{prog}
    }
}

////               DataSet Implementations         /////

impl DataSet for FullDataSet{
    fn record_iter(&self) -> Iter<DataRecord>{
        self.records.iter()
    }
    fn size(&self) -> usize{
        self.records.len()
    }
}

impl DataSet for TestDataSet{
    fn record_iter(&self) -> Iter<DataRecord>{
        self.records.iter()
    }
    fn size(&self) -> usize{
        self.records.len()
    }
}

impl DataSet for ValidationSet{
    fn record_iter(&self) -> Iter<DataRecord>{
        self.records.iter()
    }
    fn size(&self) -> usize{
        self.records.len()
    }
}
