use super::maps::ResultMap;
use evo_sys::prog::prog::Program;
use rand;
use rand::Rng;
use params;

impl ResultMap{

    pub fn is_better(&self, new_prog: &Program, old_prog: &Program) -> bool {
        match self.config.compare_prog_method {
            0 => self.simple_tie_shortest(new_prog, old_prog),
            1 => self.simple_tie_rand(new_prog, old_prog),
            2 => self.pen(new_prog, old_prog),
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


    fn pen(&self, new_prog: &Program, old_prog: &Program) -> bool{
        let period = 200_000.0;
        let mut v = self.recieved_count as f32 / period;
        v = (v.sin() + 1) / params::dataset::N_SAMPLES as f32;

        let new = new_prog.test_fit.unwrap() - v*new_prog.get_effective_len(0);
        let old = old_prog.test_fit.unwrap() - v*old_prog.get_effective_len(0);

        if new == old {
            return rand::thread_rng().gen_weighted_bool(params::evolution::REPLACE_EQ_FIT);
        }
        return new > old
    }

}