use std::ops::Range;

//pub const MAX_EVALS: u64 = 25_000;
//pub const INITIAL_POP_SIZE: u32 = 250;

pub const MAP_ROWS: usize = 50;
pub const MAP_COLS: usize = 50;

pub const N_THREADS: usize = 4;
pub const WORKER_QUEUE_SIZE: usize = 15;
pub const THREAD_POOL_MAX: u64 = 10000;

pub const MAX_REGS: usize = 128;

pub const EPS: f32 = 1e-6;

pub const DUPLICATE_TIME_OUT: u32 = 10_000; //when trying to generate new number, quit after this many times

//pub const MAX_FEATURE: u8 = 10;
pub const N_OPS: u8 = 8;

pub const DATA: &str = "inputs/data.csv";
pub const N_FOLDS: u8 = 5;
pub const N_FEATURES: u8 = 156;
pub const N_SAMPLES: usize = 389;

pub const N_POS_FOLD: usize = 30;
pub const N_NEG_FOLD: usize = 47;

pub const FOLD_SIZE: usize = N_POS_FOLD + N_NEG_FOLD; // floor(N_SAMPLES/n_fold) -> 389/5
pub const TEST_DATA_SET_SIZE: usize = FOLD_SIZE*(N_FOLDS as usize-1);

pub const POS_SAMPLE_RNG: Range<usize> = 0..152;
pub const NEG_SAMPLE_RNG: Range<usize> = 152..389;



//pub const SUB_SAMPLE_SIZE: usize = 389;

pub const NA_TOKEN: f32 = -1.0f32;




pub const MIN_FIT: f32 = -1.0f32;




// Rates are expressed as 1 in RATE chance (eg RATE = 20 => 1/20 = 4% chance)

pub const REPLACE_EQ_FIT: u32 = 100; //rate to replace best when fitness is eq

pub const INSTR_INSERT_RATE: u32 = 50; //rate to insert new instruction after copying instruction
//pub const INSTR_DUPL_RATE: u32 = 50; //rate to duplicate new instruction after copying instruction
pub const INSTR_DEL_RATE: u32 = 25; //rate to insert new instruction after copying instruction

//pub const ADD_FEAT_RATE: u32 = 10; //rate to add new feat when copying genome
//pub const REMOVE_FEAT_RATE: u32 = 10; //rate to add new feat when copying genome
//pub const SWAP_FEAT_RATE: u32 = 10;
//
//pub const ADD_COMP_REG: u32 = 10;
//pub const REMOVE_COMP_REG: u32 = 10;

pub const MUT_INSTR_COPY_RATE: u32 = 20; // was 200
