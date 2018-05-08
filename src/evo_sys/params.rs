//pub const GEN_POP_SIZE: usize = 250;
//pub const GEN_POP_GENS: u32 = 250;


// Rates are expressed as 1 in RATE chance (eg RATE = 20 => 1/20 = 4% chance)

pub const REPLACE_EQ_FIT: u32 = 2; //rate to replace best when fitness is eq

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



///     Defaults for creating random program   ////

pub const DEFAULT_INSTR_MIN: usize = 1;
pub const DEFAULT_INSTR_MAX: usize = 50;

pub const DEFAULT_CALC_REG_MIN: u8 = 100;
pub const DEFAULT_CALC_REG_MAX: u8 = 100;

pub const DEFAULT_N_OPS_MIN: u8 = 8;
pub const DEFAULT_N_OPS_MAX: u8 = 8;

pub const DEFAULT_FEAT_MIN: u8 = 155;
pub const DEFAULT_FEAT_MAX: u8 = 155;