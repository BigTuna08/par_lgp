use super::maps::ResultMap;
use evo_sys::prog::prog::Program;
use rand;
use rand::Rng;
use params;

use std::f32::consts::PI;

impl ResultMap{

    pub fn is_better(&self, new_prog: &Program, old_prog: &Program) -> bool {
        match self.config.compare_prog_method {
            0 => self.simple_tie_shortest(new_prog, old_prog),
            1 => self.simple_tie_rand(new_prog, old_prog),
            2 => self.pen_small(new_prog, old_prog),
            3 => self.pen_big(new_prog, old_prog),
            4 => self.var_pen_small(new_prog, old_prog),
            5 => self.var_pen_big(new_prog, old_prog),
            6 => self.var_pen_double(new_prog, old_prog),
            7 => self.var_pen_bigger(new_prog, old_prog),
            8 => self.var_pen_big_feats(new_prog, old_prog),
            9 => self.var_pen_bigger_feats(new_prog, old_prog),
            10 => self.var_pen_bigger_halftime(new_prog, old_prog),
            _ => panic!("Invalid compare method!! \n{:?}", self.config),
        }
    }

    fn simple_tie_shortest(&self, new_prog: &Program, old_prog: &Program) -> bool{
        if new_prog.test_fit.unwrap() == old_prog.test_fit.unwrap(){
            if new_prog.get_effective_len(0) == old_prog.get_effective_len(0){
                return rand::thread_rng().gen_weighted_bool(params::evolution::REPLACE_EQ_FIT);
            }
            else {
                return new_prog.get_effective_len(0) < old_prog.get_effective_len(0)
            }
        }
        else {
            return new_prog.test_fit.unwrap() > old_prog.test_fit.unwrap()
        }
    }

    fn simple_tie_rand(&self, new_prog: &Program, old_prog: &Program) -> bool{
        if new_prog.test_fit.unwrap() == old_prog.test_fit.unwrap(){
            return rand::thread_rng().gen_weighted_bool(params::evolution::REPLACE_EQ_FIT);
        }
        else {
            return new_prog.test_fit.unwrap() > old_prog.test_fit.unwrap()
        }
    }


    fn pen_small(&self, new_prog: &Program, old_prog: &Program) -> bool{

        let v = 1.0 / params::dataset::N_SAMPLES as f32;

        let new = new_prog.test_fit.unwrap() - v*new_prog.get_effective_len(0) as f32;
        let old = old_prog.test_fit.unwrap() - v*old_prog.get_effective_len(0) as f32;

        if new == old {
            return rand::thread_rng().gen_weighted_bool(params::evolution::REPLACE_EQ_FIT);
        }
        return new > old
    }

    fn pen_big(&self, new_prog: &Program, old_prog: &Program) -> bool{
        let period = 200_000.0;

        let v = 5.0 / params::dataset::N_SAMPLES as f32;

        let new = new_prog.test_fit.unwrap() - v*new_prog.get_effective_len(0) as f32;
        let old = old_prog.test_fit.unwrap() - v*old_prog.get_effective_len(0) as f32;

        if new == old {
            return rand::thread_rng().gen_weighted_bool(params::evolution::REPLACE_EQ_FIT);
        }
        return new > old
    }

    fn var_pen_small(&self, new_prog: &Program, old_prog: &Program) -> bool{
        let period = 200_000.0;
        let mut v = self.recieved_count as f32 / period;
        v = (v.sin() + 1.0) / params::dataset::N_SAMPLES as f32;

        let new = new_prog.test_fit.unwrap() - v*new_prog.get_effective_len(0) as f32;
        let old = old_prog.test_fit.unwrap() - v*old_prog.get_effective_len(0) as f32;

        if new == old {
            return rand::thread_rng().gen_weighted_bool(params::evolution::REPLACE_EQ_FIT);
        }
        return new > old
    }

    fn var_pen_big(&self, new_prog: &Program, old_prog: &Program) -> bool{
        let period = 500_000.0;
        let mut v = self.recieved_count as f32 / period;
        v = (v.sin() + 1.0) / params::dataset::N_SAMPLES as f32;
        v *= 10.0;

        let new = new_prog.test_fit.unwrap() - v*new_prog.get_effective_len(0) as f32;
        let old = old_prog.test_fit.unwrap() - v*old_prog.get_effective_len(0) as f32;

        if new == old {
            return rand::thread_rng().gen_weighted_bool(params::evolution::REPLACE_EQ_FIT);
        }
        return new > old
    }


    fn var_pen_double(&self, new_prog: &Program, old_prog: &Program) -> bool{
        let period = 500_000.0;
        let mut v = self.recieved_count as f32 / period;
        v = (v.sin() + 1.0) / params::dataset::N_SAMPLES as f32;
        v *= 10.0;


        let (new, old) = if ((self.recieved_count as f32 / period ) as u16 % 2)== 0{
            (new_prog.test_fit.unwrap() - v*new_prog.get_n_effective_feats(0) as f32,
                old_prog.test_fit.unwrap() - v*old_prog.get_n_effective_feats(0) as f32)
        } else {
            (new_prog.test_fit.unwrap() - v*new_prog.get_effective_len(0) as f32,
             old_prog.test_fit.unwrap() - v*old_prog.get_effective_len(0) as f32)
        };

        if new == old {
            return rand::thread_rng().gen_weighted_bool(params::evolution::REPLACE_EQ_FIT);
        }
        return new > old
    }

    fn var_pen_bigger(&self, new_prog: &Program, old_prog: &Program) -> bool{
        let period = 500_000.0;
        let mut v = self.recieved_count as f32 / period;
        v = (v.sin() + 1.0) / params::dataset::N_SAMPLES as f32;
        v *= 40.0;

        let new = new_prog.test_fit.unwrap() - v*new_prog.get_effective_len(0) as f32;
        let old = old_prog.test_fit.unwrap() - v*old_prog.get_effective_len(0) as f32;

        if new == old {
            return rand::thread_rng().gen_weighted_bool(params::evolution::REPLACE_EQ_FIT);
        }
        return new > old
    }


    fn var_pen_big_feats(&self, new_prog: &Program, old_prog: &Program) -> bool{
        let period = 500_000.0;
        let mut v = self.recieved_count as f32 / period;
        v = (v.sin() + 1.0) / params::dataset::N_SAMPLES as f32;
        v *= 10.0;

        let new = new_prog.test_fit.unwrap() - v*new_prog.get_n_effective_feats(0) as f32;
        let old = old_prog.test_fit.unwrap() - v*old_prog.get_n_effective_feats(0) as f32;

        if new == old {
            return rand::thread_rng().gen_weighted_bool(params::evolution::REPLACE_EQ_FIT);
        }
        return new > old
    }


    fn var_pen_bigger_feats(&self, new_prog: &Program, old_prog: &Program) -> bool{
        let period = 500_000.0;
        let mut v = self.recieved_count as f32 / period;
        v = (v.sin() + 1.0) / params::dataset::N_SAMPLES as f32;
        v *= 40.0;

        let new = new_prog.test_fit.unwrap() - v*new_prog.get_n_effective_feats(0) as f32;
        let old = old_prog.test_fit.unwrap() - v*old_prog.get_n_effective_feats(0) as f32;

        if new == old {
        return rand::thread_rng().gen_weighted_bool(params::evolution::REPLACE_EQ_FIT);
        }
        return new > old
    }


    fn var_pen_bigger_halftime(&self, new_prog: &Program, old_prog: &Program) -> bool{
        let period = 500_000.0;
        let mut v = self.recieved_count as f32 / period;
        v = (v.sin() + 1.0) / params::dataset::N_SAMPLES as f32;
        v *= 10.0;


        let (new, old) = if ((self.recieved_count as f32 / period ) as u16 % 2)== 0{
            (new_prog.test_fit.unwrap() - v*new_prog.get_n_effective_feats(0) as f32,
             old_prog.test_fit.unwrap() - v*old_prog.get_n_effective_feats(0) as f32)
        } else {
            (new_prog.test_fit.unwrap(), old_prog.test_fit.unwrap() )
        };

        if new == old {
            return rand::thread_rng().gen_weighted_bool(params::evolution::REPLACE_EQ_FIT);
        }
        return new > old
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
}