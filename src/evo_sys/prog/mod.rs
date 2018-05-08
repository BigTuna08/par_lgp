pub mod ops;
pub mod eval;
pub mod prog;
pub mod mutation;
mod instr;

use params as global_params;
use rand::{Rng, ThreadRng};



//returns random number in [0,n_small)U[MAX_REGS-n_big, MAX_REGS)
pub fn get_src(n_small: u8, n_big: u8, rng: &mut ThreadRng) ->u8 {
//    println!("getting src 1! nsmall{} nbig{} ", n_small, n_big);
    let mut val = rng.gen_range(0, n_small + n_big);
//    println!("getting src! nsmall{} nbig{} val{} glob{}", n_small, n_big, val, global_params::params::MAX_REGS);

    //inner math needs to be done as u16 to aviod overflows from u8. Final answer will be in 0..MAX_REGS
    if val >= n_small {val = (global_params::params::MAX_REGS as u16 - val as u16 + n_small as u16 -1) as u8}

    val
}


pub fn reg_2_feat(feat_list: &Vec<u8>, reg: &u8) -> u8{
//    println!("list-{:?} reg-{}", feat_list, reg);
    let feat_i = global_params::params::MAX_REGS - *reg as usize -1;
    feat_list[feat_i].clone()
}