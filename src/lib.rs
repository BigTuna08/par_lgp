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

pub type GenoEval = Fn(&Program) -> f32 + 'static;

pub fn heads(){
    let h = dataMgmt::dataset::get_headers(params::dataset::DATA);
    print!("headers = [");
    for n in h.iter(){
        println!("\"{}\",",n);
    }
    print!("]");
}
