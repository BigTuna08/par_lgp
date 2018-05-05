use super::super::{ResultMap, Program};
use rand;
use rand::Rng;

use dataMgmt;

use super::VarPenConfig;


impl ResultMap{

    pub fn compare(&self, new_prog: &Program, old_prog: &Program) -> bool {
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

            11 => self.var_pen_configurable_eff_len(new_prog, old_prog,  //baseline
                                                    VarPenConfig::new(0.0,
                                                                      1.0,
                                                                      3.0,
                                                                      self.config.initial_pop as u64,
                                                                      self.config.initial_pop as u64,
                                                                      self.config.total_evals)),

            12 => self.var_pen_configurable_eff_len(new_prog, old_prog, //double waves
                                                    VarPenConfig::new(0.0,
                                                                      1.0,
                                                                      6.0,
                                                                      self.config.initial_pop as u64,
                                                                      self.config.initial_pop as u64,
                                                                      self.config.total_evals)),

            13 => self.var_pen_configurable_eff_len(new_prog, old_prog, //slight bonus
                                                    VarPenConfig::new(-0.2,
                                                                      1.0,
                                                                      3.0,
                                                                      self.config.initial_pop as u64,
                                                                      self.config.initial_pop as u64,
                                                                      self.config.total_evals)),

            14 => self.var_pen_configurable_eff_len(new_prog, old_prog, //big penalty
                                                    VarPenConfig::new(0.0,
                                                                      20.0,
                                                                      3.0,
                                                                      self.config.initial_pop as u64,
                                                                      self.config.initial_pop as u64,
                                                                      self.config.total_evals)),

            _ => panic!("Invalid compare method!! \n{:?}", self.config),
        }
    }

    fn simple_tie_shortest(&self, new_prog: &Program, old_prog: &Program) -> bool{
        if new_prog.test_fit.unwrap() == old_prog.test_fit.unwrap(){
            if new_prog.get_effective_len(0) == old_prog.get_effective_len(0){
                return rand::thread_rng().gen_weighted_bool(super::super::params::REPLACE_EQ_FIT);
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
            return rand::thread_rng().gen_weighted_bool(super::super::params::REPLACE_EQ_FIT);
        }
        else {
            return new_prog.test_fit.unwrap() > old_prog.test_fit.unwrap()
        }
    }


    fn pen_small(&self, new_prog: &Program, old_prog: &Program) -> bool{

        let v = 1.0 / dataMgmt::params::N_SAMPLES as f32;

        let new = new_prog.test_fit.unwrap() - v*new_prog.get_effective_len(0) as f32;
        let old = old_prog.test_fit.unwrap() - v*old_prog.get_effective_len(0) as f32;

        if new == old {
            return rand::thread_rng().gen_weighted_bool(super::super::params::REPLACE_EQ_FIT);
        }
        return new > old
    }

    fn pen_big(&self, new_prog: &Program, old_prog: &Program) -> bool{

        let v = 5.0 / dataMgmt::params::N_SAMPLES as f32;

        let new = new_prog.test_fit.unwrap() - v*new_prog.get_effective_len(0) as f32;
        let old = old_prog.test_fit.unwrap() - v*old_prog.get_effective_len(0) as f32;

        if new == old {
            return rand::thread_rng().gen_weighted_bool(super::super::params::REPLACE_EQ_FIT);
        }
        return new > old
    }

    fn var_pen_small(&self, new_prog: &Program, old_prog: &Program) -> bool{
        let period = 200_000.0;
        let mut v = self.recieved_count as f32 / period;
        v = (v.sin() + 1.0) / dataMgmt::params::N_SAMPLES as f32;

        let new = new_prog.test_fit.unwrap() - v*new_prog.get_effective_len(0) as f32;
        let old = old_prog.test_fit.unwrap() - v*old_prog.get_effective_len(0) as f32;

        if new == old {
            return rand::thread_rng().gen_weighted_bool(super::super::params::REPLACE_EQ_FIT);
        }
        return new > old
    }

    fn var_pen_big(&self, new_prog: &Program, old_prog: &Program) -> bool{
        let period = 500_000.0;
        let mut v = self.recieved_count as f32 / period;
        v = (v.sin() + 1.0) / dataMgmt::params::N_SAMPLES as f32;
        v *= 10.0;

        let new = new_prog.test_fit.unwrap() - v*new_prog.get_effective_len(0) as f32;
        let old = old_prog.test_fit.unwrap() - v*old_prog.get_effective_len(0) as f32;

        if new == old {
            return rand::thread_rng().gen_weighted_bool(super::super::params::REPLACE_EQ_FIT);
        }
        return new > old
    }


    fn var_pen_double(&self, new_prog: &Program, old_prog: &Program) -> bool{
        let period = 500_000.0;
        let mut v = self.recieved_count as f32 / period;
        v = (v.sin() + 1.0) / dataMgmt::params::N_SAMPLES as f32;
        v *= 10.0;


        let (new, old) = if ((self.recieved_count as f32 / period ) as u16 % 2)== 0{
            (new_prog.test_fit.unwrap() - v*new_prog.get_n_effective_feats(0) as f32,
                old_prog.test_fit.unwrap() - v*old_prog.get_n_effective_feats(0) as f32)
        } else {
            (new_prog.test_fit.unwrap() - v*new_prog.get_effective_len(0) as f32,
             old_prog.test_fit.unwrap() - v*old_prog.get_effective_len(0) as f32)
        };

        if new == old {
            return rand::thread_rng().gen_weighted_bool(super::super::params::REPLACE_EQ_FIT);
        }
        return new > old
    }

    fn var_pen_bigger(&self, new_prog: &Program, old_prog: &Program) -> bool{
        let period = 500_000.0;
        let mut v = self.recieved_count as f32 / period;
        v = (v.sin() + 1.0) / dataMgmt::params::N_SAMPLES as f32;
        v *= 40.0;

        let new = new_prog.test_fit.unwrap() - v*new_prog.get_effective_len(0) as f32;
        let old = old_prog.test_fit.unwrap() - v*old_prog.get_effective_len(0) as f32;

        if new == old {
            return rand::thread_rng().gen_weighted_bool(super::super::params::REPLACE_EQ_FIT);
        }
        return new > old
    }


    fn var_pen_big_feats(&self, new_prog: &Program, old_prog: &Program) -> bool{
        let period = 500_000.0;
        let mut v = self.recieved_count as f32 / period;
        v = (v.sin() + 1.0) / dataMgmt::params::N_SAMPLES as f32;
        v *= 10.0;

        let new = new_prog.test_fit.unwrap() - v*new_prog.get_n_effective_feats(0) as f32;
        let old = old_prog.test_fit.unwrap() - v*old_prog.get_n_effective_feats(0) as f32;

        if new == old {
            return rand::thread_rng().gen_weighted_bool(super::super::params::REPLACE_EQ_FIT);
        }
        return new > old
    }


    fn var_pen_bigger_feats(&self, new_prog: &Program, old_prog: &Program) -> bool{
        let period = 500_000.0;
        let mut v = self.recieved_count as f32 / period;
        v = (v.sin() + 1.0) / dataMgmt::params::N_SAMPLES as f32;
        v *= 40.0;

        let new = new_prog.test_fit.unwrap() - v*new_prog.get_n_effective_feats(0) as f32;
        let old = old_prog.test_fit.unwrap() - v*old_prog.get_n_effective_feats(0) as f32;

        if new == old {
        return rand::thread_rng().gen_weighted_bool(super::super::params::REPLACE_EQ_FIT);
        }
        return new > old
    }


    fn var_pen_bigger_halftime(&self, new_prog: &Program, old_prog: &Program) -> bool{
        let period = 500_000.0;
        let mut v = self.recieved_count as f32 / period;
        v = (v.sin() + 1.0) / dataMgmt::params::N_SAMPLES as f32;
        v *= 10.0;


        let (new, old) = if ((self.recieved_count as f32 / period ) as u16 % 2)== 0{
            (new_prog.test_fit.unwrap() - v*new_prog.get_n_effective_feats(0) as f32,
             old_prog.test_fit.unwrap() - v*old_prog.get_n_effective_feats(0) as f32)
        } else {
            (new_prog.test_fit.unwrap(), old_prog.test_fit.unwrap() )
        };

        if new == old {
            return rand::thread_rng().gen_weighted_bool(super::super::params::REPLACE_EQ_FIT);
        }
        return new > old
    }

    fn var_pen_configurable_eff_len(&self, new_prog: &Program, old_prog: &Program, config: VarPenConfig) -> bool {
        let pen = config.penalty_at(self.recieved_count);

        let new = new_prog.test_fit.unwrap() - pen*new_prog.get_effective_len(0) as f32;
        let old = old_prog.test_fit.unwrap() - pen*old_prog.get_effective_len(0) as f32;

        if new == old { //random
            return rand::thread_rng().gen_weighted_bool(super::super::params::REPLACE_EQ_FIT);
        }
        return new > old
    }

    fn var_pen_configurable_eff_feats(&self, new_prog: &Program, old_prog: &Program, config: VarPenConfig) -> bool {
        let pen = config.penalty_at(self.recieved_count);

        let new = new_prog.test_fit.unwrap() - pen*new_prog.get_n_effective_feats(0) as f32;
        let old = old_prog.test_fit.unwrap() - pen*old_prog.get_n_effective_feats(0) as f32;

        if new == old { //random
            return rand::thread_rng().gen_weighted_bool(super::super::params::REPLACE_EQ_FIT);
        }
        return new > old
    }

}