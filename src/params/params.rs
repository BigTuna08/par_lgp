//pub const MAP_ROWS: usize = 50;
//pub const MAP_COLS: usize = 50;
pub const MAP_ROWS: usize = 20;
pub const MAP_COLS: usize = 32;

pub const MAX_REGS: usize = 128; //was 128, risk of crashing if less than N_FEATURES, during feature loading. if > 256 will also crash!

pub const N_OPS: u8 = 8;


pub const EPS: f32 = 1e-6;
pub const DUPLICATE_TIME_OUT: u32 = 100_000; //when trying to generate new number, quit after this many times


pub const NA_TOKEN: f32 = -1.0f32;
pub const MIN_FIT: f32 = -1.0f32;
