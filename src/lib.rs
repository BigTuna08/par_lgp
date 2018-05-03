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

use evo_sys::prog::prog::Program;

use rand::thread_rng;
use rand::Rng;



pub type GenoEval = Fn(&Program) -> f32 + 'static;






pub fn heads(){
    let h = dataMgmt::dataset::get_headers(params::dataset::DATA);
    print!("headers = [");
    for n in h.iter(){
        println!("\"{}\",",n);
    }
    print!("]");
}

pub fn test_a2s(f: &Fn(&[u8])->String, iters: usize){
    for _ in 0..iters{
        let mut a = [0u8; 200];
        for val in a.iter_mut(){
            *val = thread_rng().gen()
        }
        let s = f(&a);
    }
}