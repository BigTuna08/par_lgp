pub const MAP_ROWS: usize = 50;
pub const MAP_COLS: usize = 50;
pub const MAX_REGS: usize = 200; //was 128, risk of crashing if less than N_FEATURES, during feature loading

pub const N_OPS: u8 = 8;

pub const N_THREADS: usize = 4;
pub const WORKER_QUEUE_SIZE: usize = 15;
pub const THREAD_POOL_MAX: u64 = 10000;


pub const EPS: f32 = 1e-6;
pub const DUPLICATE_TIME_OUT: u32 = 10_000; //when trying to generate new number, quit after this many times


pub const NA_TOKEN: f32 = -1.0f32;
pub const MIN_FIT: f32 = -1.0f32;