pub mod ops;
pub mod eval;
pub mod prog;
pub mod mutation;
mod instr;

use params as global_params;
use rand::{Rng, ThreadRng};



//returns random number in [0,n_small)U[MAX_REGS-n_big, MAX_REGS)
pub fn get_src(n_small: u8, n_big: u8, rng: &mut ThreadRng) ->u8 {
    let mut val = rng.gen_range(0, n_small + n_big);

    if val >= n_small {val = global_params::params::MAX_REGS as u8 - val + n_small-1 }

    val
}


pub fn reg_2_feat(feat_list: &Vec<u8>, reg: &u8) -> u8{
    let feat_i = global_params::params::MAX_REGS - *reg as usize -1;
    feat_list[feat_i].clone()
}