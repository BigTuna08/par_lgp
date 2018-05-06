pub mod maps;
pub mod selectors;
pub mod comparers;

use std::fs::File;
use std::io::Write;



use std::f32::consts::PI;

//pub trait Population {
//
//    fn is_finished(&self) -> bool;
//    fn can_send(&self) -> bool;
//
//    fn next_new_prog(&mut self) -> Program; //mut so sent count is incremented
//    fn try_put(&mut self, new_entry: EvalResult);
//
//    fn update_cv(&mut self);
//
//    fn log_full(&self, logger: &mut Logger); //Continuous logging.
//    fn write_pop_info(&self, file_name: &str, eval: PopEval); //end of fold log
//    fn write_genos(&self, file_name: &str); //end of fold log
//
//}
//
//
//pub trait PopMap: Population{
//    fn get_config(&self) -> &PopConfig;
//
//    fn select_cell(&self, prog: &Program) -> (usize, usize);
//    fn compare_program(&self, new_prog: &Program, old_prog: &Program) -> bool;
//    fn is_in_bounds(&self, inds: &(usize,usize))-> bool;
//    fn get(&self, inds:  &(usize,usize)) -> &Option<Program>;
//    fn put(&mut self,  prog: Program, inds:  &(usize,usize));
//}
#[derive(Debug)]
pub struct VarPenConfig{
    v_stretch: f32, // ie amplitude
    h_stretch: f32, // from period
    v_shift: f32, //vertical shift of wave
    h_shift: f32, //horizontal shift
    protect_start: u64,
    protect_end: u64,
}


impl VarPenConfig{
    pub fn new(min: f32, max: f32, n_waves: f32, protect_start: u64, protect_end: u64, total_evals: u64) -> VarPenConfig{
        if min > max || min > 0.0 || max < 0.0 {
            panic!("invalid min/max values: min {} max {}\n0 must be in between these. This will be \
            made more stable in the future, so 0 is not needed in range", min, max);
        }
        let n_pen_evals = total_evals - protect_start - protect_end;
        let v_stretch = (max - min)/2.0;
        let v_shift = max - v_stretch;
        let h_stretch = 2.0*PI*n_waves/n_pen_evals as f32;

        let h_shift = (-v_shift/v_stretch).asin()/h_stretch - protect_start as f32;

        VarPenConfig{
            v_stretch, h_stretch, v_shift, h_shift, protect_start, protect_end:total_evals-protect_end,
        }
    }

    pub fn penalty_at(&self, current_eval: u64)-> f32{
       
        if current_eval < self.protect_start || current_eval > self.protect_end {
            return 0.0;
        }
        let inside = self.h_stretch*(current_eval as f32+self.h_shift);
        self.v_stretch*inside.sin() + self.v_shift
    }
}



pub fn test(){
    let config = VarPenConfig::new(-1.0, 4.0, 3.0, 10_000, 10_000, 100_000);

    println!("{:?}", config.penalty_at(10_000));
    println!("{:?}", config.penalty_at(10_011));
    println!("{:?}", config.penalty_at(20_000));
    println!("{:?}", config.penalty_at(30_000));
    println!("{:?}", config.penalty_at(40_000));
    println!("{:?}", config.penalty_at(89_990));
    println!("{:?}", config.penalty_at(90_000));
    println!("{:?}", config.penalty_at(90_001));
    println!("{:?}", config.penalty_at(90_010));
}


//    fn var_pen(&self, new_prog: &Program, old_prog: &Program, min_pen: f32, max_pen: f32, n_waves: f32, protect_start: u64, protect_end: u64 ) -> bool{
//        let wave_input = (self.config.total_evals- self.recieved_count) as f32;
//
//        let period = self.config.total_evals - protect_start - protect_end;
//        let period = (period as f32)/n_waves;
//
//        let mut v = (2.0*PI*wave_input/period).sin();
//
//
//
//        let vert_strech = (max_pen-min_pen)/2.0;
//
//        let vert_trans = max_pen - vert_strech;
//
//    }


pub struct PopStats {
    pub best: f32,
    pub worst: f32,
    pub ave: f64,
    pub sd: f64,
//    pub count: f32,
}


impl PopStats {
    pub fn to_string(&self)->String{
        format!("best: {}\nworst: {}\nave: {}", self.best, self.worst, self.ave)
    }

    pub fn write_update(&self, f: &mut File, n_evals: u64){
        f.write(b"*after ");
        f.write(n_evals.to_string().as_bytes());
        f.write(b" evaluations\n");
        f.write(self.to_string().as_bytes());
        f.write(b"\n\n");
        f.flush();
    }
}




//
//pub enum PutResult{
//    Failed,
//    Equal,
//    Improvement
//}
