use dataMgmt;
use params as global_params;
use rand;
use rand::{Rng};
use super::super::{Program};

impl Program{

    pub fn test_mutate_copy(&self) -> Program{

        let n = rand::thread_rng().gen_range(0, 25);
        match n {
            0 => self.ins_instr_copy(),
            1 => self.del_instr_copy(),
            2 => self.ins_comp_copy(),
            3 => self.del_comp_copy(),
            4 => self.ins_feat_copy(),
            5 => self.del_feat_copy(),
            6 => self.swap_feat_copy(),
            _ => self.mut_instr_copy(),
        }

    }


    pub fn mut_instr_copy(&self) -> Program{
        let features = self.features.clone();
        let n_calc_regs = self.n_calc_regs;
        let mut rng = rand::thread_rng();

        let instructions = self.instructions.iter().map(|instr| {
            if rand::thread_rng().gen_weighted_bool(super::super::params::MUT_INSTR_COPY_RATE){
                instr.mutate_copy(&self, &mut rng)
            }
                else {
                    instr.clone()
                }
        }).collect();

        Program{n_calc_regs, features, instructions, test_fit:None, cv_fit:None}
    }


    pub fn ins_instr_copy(&self) -> Program{
        let features = self.features.clone();
        let n_calc_regs = self.n_calc_regs;
        let mut rng = rand::thread_rng();

        let mut instructions = Vec::with_capacity(self.instructions.len() + 10 ); //random 10 to allow for insertions

        for instr in self.instructions.iter() {
            instructions.push(instr.clone());
            if rng.gen_weighted_bool(super::super::params::INSTR_INSERT_RATE) {
                instructions.push(self.rand_instr(&mut rng))
            }
        }
        instructions.shrink_to_fit();

        Program{n_calc_regs, features, instructions, test_fit:None, cv_fit:None}
    }


    pub fn del_instr_copy(&self) -> Program {
        let features = self.features.clone();
        let n_calc_regs = self.n_calc_regs;
        let mut rng = rand::thread_rng();

        let mut instructions = Vec::with_capacity(self.instructions.len());

        for instr in self.instructions.iter() {

            if !rng.gen_weighted_bool(super::super::params::INSTR_DEL_RATE) {
                instructions.push(instr.clone());
            }
        }
        instructions.shrink_to_fit();
        Program{n_calc_regs, features, instructions, test_fit:None, cv_fit:None}
    }


    // very simple now, only add reg as option, does not change instructions
    // in future may want to find a way to distribute some work to new reg while
    // maintaining the programs correctness
    pub fn ins_comp_copy(&self) -> Program{
        let features = self.features.clone();
        let n_calc_regs = self.n_calc_regs +1;

        let mut instructions = Vec::with_capacity(self.instructions.len());

        for instr in self.instructions.iter() {
            instructions.push(instr.clone());
        }

        Program{n_calc_regs, features, instructions, test_fit:None, cv_fit:None}
    }


    //very simple now, just rm instr if it has deleted reg
    pub fn del_comp_copy(&self) -> Program{
        if self.n_calc_regs == 1 {
            return Program::new_random(self.instructions.len(), 5, global_params::params::N_OPS, self.features.len() as u8)
        }
        let features = self.features.clone();
        let n_calc_regs = self.n_calc_regs -1;

        let mut instructions = Vec::with_capacity(self.instructions.len());

        for instr in self.instructions.iter() {
            if !instr.contains_reg(n_calc_regs){
                instructions.push(instr.clone());
            }
        }
        instructions.shrink_to_fit();
        Program{n_calc_regs, features, instructions, test_fit:None, cv_fit:None}
    }


    pub fn ins_feat_copy(&self) -> Program{
        let mut rng = rand::thread_rng();
        let mut features = self.features.clone();

        if self.features.len() == dataMgmt::params::N_FEATURES as usize{ //just do micro mutation
            return self.mut_instr_copy()
        }

        let mut new_feat = rng.gen_range(0, dataMgmt::params::N_FEATURES);
        let mut tries = 0;
        while features.contains(&new_feat) {
            new_feat =  rng.gen_range(0, dataMgmt::params::N_FEATURES);
            tries += 1;
            if tries > global_params::params::DUPLICATE_TIME_OUT { panic!("Error getting non dupicate!, {:?}", &self)}
        }
        features.push(new_feat);

        let n_calc_regs = self.n_calc_regs;


        let mut instructions = Vec::with_capacity(self.instructions.len());

        for instr in self.instructions.iter() {
            instructions.push(instr.clone());
        }

        Program{n_calc_regs, features, instructions, test_fit:None, cv_fit:None}
    }


    //very simple now, just rm instr if it has deleted reg
    pub fn del_feat_copy(&self) -> Program{
        if self.features.len() == 1 { //do micro instead
            return self.mut_instr_copy()
        }

        let mut features = self.features.clone();
        features.pop();
        let removed_reg = (global_params::params::MAX_REGS - 1- features.len()) as u8;

        let n_calc_regs = self.n_calc_regs;


        let mut instructions = Vec::with_capacity(self.instructions.len());

        for instr in self.instructions.iter() {
            if !instr.contains_reg(removed_reg){
                instructions.push(instr.clone());
            }
        }
        instructions.shrink_to_fit();
        Program{n_calc_regs, features, instructions, test_fit:None, cv_fit:None}
    }


    pub fn swap_feat_copy(&self) -> Program{

        if self.features.len() == dataMgmt::params::N_FEATURES as usize { //just do micro mutation
            return self.mut_instr_copy()
        }

        let mut rng = rand::thread_rng();
        let mut features = self.features.clone();

        let mut new_feat = rng.gen_range(0, dataMgmt::params::N_FEATURES);
        let mut tries = 0;
        while features.contains(&new_feat) {
            new_feat =  rng.gen_range(0, dataMgmt::params::N_FEATURES);
            tries += 1;
            if tries > global_params::params::DUPLICATE_TIME_OUT { panic!("Error getting non dupicate!, {:?}", &self)}
        }


        let to_replace = rng.gen_range(0, features.len());
        features[to_replace] = new_feat;

        let n_calc_regs = self.n_calc_regs;

        let mut instructions = Vec::with_capacity(self.instructions.len());

        for instr in self.instructions.iter() {
            instructions.push(instr.clone());
        }

        Program{n_calc_regs, features, instructions, test_fit:None, cv_fit:None}
    }
}