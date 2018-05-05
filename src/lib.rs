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

use evo_sys::Program;



pub type GenoEval = Fn(&Program) -> f32 + 'static;




