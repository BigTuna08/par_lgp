extern crate csv;
extern crate indexmap;
extern crate rand;
extern crate serde;
extern crate time;

pub mod params;
pub mod evo_sys;
pub mod threading;
pub mod dataMgmt;
pub mod experiments;
pub mod config;
pub mod serv;

use evo_sys::Program;



pub type GenoEval = Fn(&Program) -> f32 + 'static;

#[derive(Debug)]
pub struct CoreConfig{
    pop_config: PopInfo,
    out_folder: String,
    data_file: String,
    compare_prog_method: u8,    // multi !
    mutate_method: u8,
    n_iterations: u32,
}

impl CoreConfig{
    pub fn create_result_map_config(&self) -> ResultMapConfig{
        match self.pop_config {
            PopInfo::Map(ref info) => {
                ResultMapConfig{
                    out_folder: self.out_folder.clone(),
                    data_file: self.data_file.clone(),
                    compare_prog_method: self.compare_prog_method,
                    mutate_method: self.mutate_method,
                    n_iterations: self.n_iterations,

                    select_cell_method: info.select_cell_method,
                    initial_pop: info.initial_pop,
                    n_evals: info.n_evals,
                    prog_defaults: config::process_prog_defaults("prog_config.txt")
                }
            },
            PopInfo::Gen(_) => panic!("Not in gen mode!!")
        }
    }

    pub fn create_gen_pop_config(&self) -> GenPopConfig{
        match self.pop_config {
            PopInfo::Map(_) => panic!("Not in map mode!!"),
            PopInfo::Gen(ref info) => {
                GenPopConfig{
                    out_folder: self.out_folder.clone(),
                    data_file: self.data_file.clone(),
                    compare_prog_method: self.compare_prog_method,
                    mutate_method: self.mutate_method,
                    n_iterations: self.n_iterations,

                    tourn_size: info.tourn_size,
                    total_gens: info.total_gens,
                    random_gens: info.random_gens,
                    prog_defaults: config::process_prog_defaults("prog_config.txt")
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum PopInfo {
    Map(MapInfo),
    Gen(GenPopInfo),
}

#[derive(Debug)]
pub struct MapInfo {
    select_cell_method: u8,
    initial_pop: u32,
    n_evals: u64,
}

#[derive(Debug)]
pub struct GenPopInfo {
    tourn_size: u16,
    total_gens: u32,
    random_gens: u32,
}

#[derive(Debug)]
pub struct ResultMapConfig{
    out_folder: String,
    data_file: String,
    compare_prog_method: u8,    // multi !
    mutate_method: u8,
    n_iterations: u32,

    select_cell_method: u8,
    initial_pop: u32,
    n_evals: u64,
    prog_defaults: ProgDefaults,
}


#[derive(Debug)]
pub struct GenPopConfig{
    out_folder: String,
    data_file: String,
    compare_prog_method: u8,    // multi !
    mutate_method: u8,
    n_iterations: u32,

    tourn_size: u16,
    total_gens: u32,
    random_gens: u32,
    prog_defaults: ProgDefaults,
}

#[derive(Debug)]
pub struct ProgDefaults{
    pub INITIAL_INSTR_MIN: usize,
    pub INITIAL_INSTR_MAX: usize,

    pub INITIAL_CALC_REG_MIN: u8,
    pub INITIAL_CALC_REG_MAX: u8,

    pub INITIAL_N_OPS_MIN: u8,
    pub INITIAL_N_OPS_MAX: u8,

    pub INITIAL_FEAT_MIN: u8,
    pub INITIAL_FEAT_MAX: u8,
}

#[derive(Copy, Clone, Debug)]
pub enum Mode{
    Map,
    Gen,
}

#[derive(Debug)]
pub struct ConfigFile{
    mode: Mode,
    out_folder: String,
    data_file: String,
    n_iterations: u32,
    mutate_methods: Vec<u8>,
    compare_methods: Vec<u8>,

    n_evals: Option<Vec<u64>>,
    inital_pop_size: Option<Vec<u32>>,
    map_methods: Option<Vec<u8>>,

    total_gens: Option<Vec<u32>>,
    random_gens: Option<Vec<u32>>,
    tourn_sizes: Option<Vec<u16>>,
}


#[derive(Debug)]
pub struct Runner{
    config: ConfigFile,
    mode: Mode,
    mutate_i: usize,
    compare_i: usize,

    started: bool,

    //vec 1..3 have different meanings based on mode
    vec_1_i: usize,  //index of n_evals or total_gens
    vec_2_i: usize, //index of inital_pop_size or init_gens
    vec_3_i: usize, //index of map_methods or tourn_size

}




