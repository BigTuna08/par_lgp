use std::ops::Range;

//pub const DATA: &str = "inputs/data3.csv";
//pub const N_FOLDS: u8 = 5;
//pub const N_FEATURES: u8 = 156;
//pub const N_SAMPLES: usize = 389;
//
//pub const N_POS_FOLD: usize = 30;
//pub const N_NEG_FOLD: usize = 47;
//
//pub const FOLD_SIZE: usize = N_POS_FOLD + N_NEG_FOLD; // floor(N_SAMPLES/n_fold) -> 389/5
//pub const TEST_DATA_SET_SIZE: usize = FOLD_SIZE*(N_FOLDS as usize-1);
//
//pub const POS_SAMPLE_RNG: Range<usize> = 0..152;
//pub const NEG_SAMPLE_RNG: Range<usize> = 152..389;


pub const N_FOLDS: u8 = 3;
pub const N_FEATURES: u8 = 5;
pub const N_SAMPLES: usize = 174;

pub const N_POS_FOLD: usize = 9; //29 totaal
pub const N_NEG_FOLD: usize = 48; // 145

pub const FOLD_SIZE: usize = N_POS_FOLD + N_NEG_FOLD; // floor(N_SAMPLES/n_fold) -> 389/5
pub const TEST_DATA_SET_SIZE: usize = FOLD_SIZE*(N_FOLDS as usize-1);

pub const POS_SAMPLE_RNG: Range<usize> = 145..174;
pub const NEG_SAMPLE_RNG: Range<usize> = 0..145;