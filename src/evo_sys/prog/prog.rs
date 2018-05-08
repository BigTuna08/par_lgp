//extern crate rand;

use dataMgmt;
use params as global_params;
use rand::{Rng, seq, thread_rng};

use std::collections::HashSet;
use std::fs::File;
use std::io::Write;

use super::ops;
use super::super::{Program, Instruction, params};




impl Program{

    ////                 New Program Methods            ////

    pub fn new_default_range() -> Program{
        Program::new_random_range(params::DEFAULT_INSTR_MIN, params::DEFAULT_INSTR_MAX,
                                  params::DEFAULT_CALC_REG_MIN, params::DEFAULT_CALC_REG_MAX,
                                  params::DEFAULT_N_OPS_MIN, params::DEFAULT_N_OPS_MAX,
                                  params::DEFAULT_FEAT_MIN, params::DEFAULT_FEAT_MAX)
    }


    pub fn new_random_range(instr_min: usize, instr_max: usize, calc_regs_min: u8,
                            calc_reg_max: u8, ops_min: u8, ops_max: u8, feats_min: u8, feats_max: u8) -> Program {

        let mut rng = thread_rng();

        Program::new_random(rng.gen_range(instr_min, instr_max +1),
                                rng.gen_range(calc_regs_min, calc_reg_max+1),
                                    rng.gen_range(ops_min, ops_max+1),
                                    rng.gen_range(feats_min, feats_max+1))

    }


    pub fn new_random(n_instr: usize, n_calc_regs: u8, n_ops: u8, n_feats: u8) -> Program{
        let mut rng = thread_rng();

        let features = seq::sample_iter(&mut rng, 0..dataMgmt::params::N_FEATURES, n_feats as usize).unwrap();
        let mut instructions = Vec::with_capacity(n_instr);

        for _ in 0..n_instr {
            instructions.push(Instruction::new_random(n_calc_regs, n_feats, n_ops, &mut rng));
        }

        Program{ n_calc_regs, features, instructions, test_fit:None, cv_fit:None}
    }


    //return a new empty program
    pub fn new_empty() -> Program{
        Program{n_calc_regs:0,
            features:Vec::new(),
            instructions:Vec::new(),
            test_fit: Some(global_params::params::MIN_FIT),
            cv_fit:Some(global_params::params::MIN_FIT), }
    }


    ////                 For Evaluating            ////


    pub fn create_compressed(&self) -> Program{
        let instr_i = self.get_important_instrs(0);
        let instrs: Vec<Instruction> = instr_i.into_iter().map(|i| {self.instructions[i].clone()}).collect();
        Program{
            n_calc_regs: 0,
            features: Vec::new(),
            instructions: instrs,
            test_fit:None,
            cv_fit: None,
        }
    }


    pub fn execute_instructions(&self, mut regs: [f32; global_params::params::MAX_REGS]) ->f32{
        let mut skip_count = 0u8; // used to implement branches

        for instr in self.instructions.iter() {
            if skip_count > 0 {
                skip_count -= 1;
                continue;
            }
            let result = ops::OPS[instr.op as usize](regs[instr.src1 as usize], regs[instr.src2 as usize]);
            match instr.op {
                0 ... 5 => regs[instr.dest as usize] = result, //simple register transfer
                6 => if result < 0.0 {skip_count = instr.src2}, //if false, skip next n, use direct constant
                7 => if result < 0.0 {skip_count = 1}, //false skip next 1
                _ => panic!("invalid op! {:?}", &instr)
            }
        }
        regs[0]
    }


    ////                Getters           ////

    //return # of effective instructions
    pub fn get_effective_len(&self, return_reg_ind: u8) -> usize{
        let mut eff_regs = HashSet::new();
        let mut count = 0;
        eff_regs.insert(return_reg_ind);
        for instr in self.instructions.iter().rev(){
            if eff_regs.contains(&instr.dest) {
                eff_regs.insert(instr.src1);
                eff_regs.insert(instr.src2);
                count += 1;
            }
        }
        count
    }


    //return # of effective instructions
    pub fn get_abs_len(&self) -> usize{
        self.instructions.len()
    }


    //returns list of line numbers of effective instr, starting from 0
    pub fn get_effective_instrs(&self, return_reg_ind: u8) -> Vec<usize>{
        let mut eff_regs = HashSet::new();
        let mut eff_instrs = Vec::new();
        eff_regs.insert(return_reg_ind);
        for (i, instr) in self.instructions.iter().enumerate().rev(){
            if eff_regs.contains(&instr.dest) {
                eff_regs.insert(instr.src1);
                eff_regs.insert(instr.src2);
                eff_instrs.push(i);
            }
        }
        eff_instrs.sort();
        eff_instrs
    }


    // like effective, but also will always include instructions after
    // branch statements. The output of only running these instrcutions should
    // be identical to running all instructions.
    pub fn get_important_instrs(&self, return_reg_ind: u8) -> Vec<usize>{
        let mut eff_regs = HashSet::new();
        let mut eff_instrs = HashSet::new();
        eff_regs.insert(return_reg_ind);
        for (i, instr) in self.instructions.iter().enumerate().rev(){
            if eff_regs.contains(&instr.dest) {
                eff_regs.insert(instr.src1);
                eff_regs.insert(instr.src2);
                eff_instrs.insert(i);
            }
            if instr.is_branch() && i < self.instructions.len()-1{ //dont do for last instr;
                eff_instrs.insert(i+1);
            }
        }
        let mut eff_instrs: Vec<usize>  = eff_instrs.into_iter().collect();
        eff_instrs.sort();
        eff_instrs
    }


    pub fn get_n_effective_feats(&self, return_reg_ind: u8) -> usize{
        let mut eff_regs = HashSet::new();
        eff_regs.insert(return_reg_ind);
        for instr in self.instructions.iter().rev(){
            if eff_regs.contains(&instr.dest) {
                eff_regs.insert(instr.src1);
                eff_regs.insert(instr.src2);
            }
        }
        eff_regs.into_iter().fold(0, |acc, x| if x >= (global_params::params::MAX_REGS - self.features.len()) as u8 {acc+1} else {acc})
    }

    pub fn get_effective_feats(&self, return_reg_ind: u8) -> HashSet<u8>{

        let mut eff_regs = HashSet::new();
        eff_regs.insert(return_reg_ind);
//        println!("first ind={} len={}", return_reg_ind, eff_regs.len());
        for instr in self.instructions.iter().rev(){
            if eff_regs.contains(&instr.dest) {
                eff_regs.insert(instr.src1);
                eff_regs.insert(instr.src2);
            }
        }
//        println!("second ind={} len={} regs{:?}", return_reg_ind, eff_regs.len(), &eff_regs);
        eff_regs.retain(|&x|  x >= (global_params::params::MAX_REGS - self.features.len()) as u8);
//        println!("thrid ind={} len={}", return_reg_ind, eff_regs.len());
        eff_regs.iter().map(|x| super::reg_2_feat(&self.features, x)).collect()
    }


    pub fn get_n_effective_comp_regs(&self, return_reg_ind: u8) -> usize{
        let mut eff_regs = HashSet::new();
        eff_regs.insert(return_reg_ind);
        for instr in self.instructions.iter().rev(){
            if eff_regs.contains(&instr.dest) {
                eff_regs.insert(instr.src1);
                eff_regs.insert(instr.src2);
            }
        }
        eff_regs.into_iter().fold(0, |acc, x| if x < self.n_calc_regs {acc+1} else {acc})
    }



    ////                 For Logging            ////

    pub fn string_instr(&self, instr: &Instruction) -> String{
        let n_feats = self.features.len();
        let src1 =
            if instr.src1 as usize >= (global_params::params::MAX_REGS - n_feats) {
                let d = (global_params::params::MAX_REGS - instr.src1 as usize) -1; //0..n_feats
                let fest_num = self.features[d];
                format!("{}",&dataMgmt::metabolites::DATA_HEADERS[fest_num as usize])
            }else {
                format!("${}",instr.src1)
            };

        let src2 =
            if instr.src2 as usize >= (global_params::params::MAX_REGS - n_feats) {
                let d = global_params::params::MAX_REGS - instr.src2 as usize - 1; //0..n_feats
                let fest_num = self.features[d];
                format!("{}",&dataMgmt::metabolites::DATA_HEADERS[fest_num as usize])
            }else {
                format!("${}",instr.src2)
            };
        format!("${}\t=\t{}\t{}\t{}", instr.dest, ops::OPS_NAMES[instr.op as usize], src1, src2)
    }

    pub fn feat_str(&self)->String{
        self.features.iter().fold(String::new(),
                     |mut acc, &x| {acc.push_str(&dataMgmt::metabolites::DATA_HEADERS[x as usize]); acc.push_str("\t"); acc} )
    }


    pub fn print_self_words(&self){
        println!("{}", self.n_calc_regs);
        println!("{}", self.feat_str());
        for instr in self.instructions.iter(){
             println!("{}",self.string_instr(instr));
        }
    }

    pub fn write_self_words(&self, f: &mut File){

        f.write(b"#Len: ");
        f.write(self.instructions.len().to_string().as_bytes());
        f.write(b"\n");
        f.write(b"#Eff Len: ");
        f.write(self.get_effective_len(0).to_string().as_bytes());
        f.write(b"\n");
        f.write(b"#Eff feats: ");
        f.write(self.get_n_effective_feats(0).to_string().as_bytes());
        f.write(b"\n");
        f.write(b"#Eff comp regs: ");
        f.write(self.get_n_effective_comp_regs(0).to_string().as_bytes());
        f.write(b"\n*");

        f.write(self.n_calc_regs.to_string().as_bytes());
        f.write(b"\n*");
        let feat_str = self.feat_str();
        f.write(feat_str.as_bytes());
        f.write(b"\n");
        for instr in self.instructions.iter(){
            let instr_str = self.string_instr(instr);
            f.write(instr_str.as_bytes());
            f.write(b"\n");
        }
        f.write(b"\n");
    }


    pub fn write_effective_self_words(&self, f: &mut File){
        f.write(b"#Eff feats: ");
        let eff_feats = self.get_effective_feats(0).iter().fold(String::new(), |acc, x| { format!("{}\t{}", acc, x)} );
        f.write(eff_feats.as_bytes());
        f.write(b"\n");
        f.write(b"#Eff comp regs: ");
        f.write(self.get_n_effective_comp_regs(0).to_string().as_bytes());
        f.write(b"\n*");
        f.write(self.n_calc_regs.to_string().as_bytes());
        f.write(b"\n*");
        let feat_str = self.feat_str();
        f.write(feat_str.as_bytes());
        f.write(b"\n");
        for instr_i in self.get_effective_instrs(0){
            let instr = self.instructions[instr_i];
            let instr_str = self.string_instr(&instr);
            f.write(instr_str.as_bytes());
            f.write(b"\n");
        }
        f.write(b"\n");
    }

}










