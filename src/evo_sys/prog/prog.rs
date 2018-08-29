//extern crate rand;

use dataMgmt;
use dataMgmt::metabolites;
use params as global_params;
use rand::{Rng, seq, thread_rng};

use std::collections::HashSet;
use std::fs::File;
use std::io::Write;

use super::ops;
use super::super::{Program, Instruction, ExecutionRegArray, params, InstructionType};




impl Program{

    ////                 New Program Methods            ////



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

        Program{ n_calc_regs, features, instructions, test_fit:None, cv_fit:None, pos_missed: None,
            neg_missed: None,}
    }


    //return a new empty program
    pub fn new_empty() -> Program{
        Program{n_calc_regs:0,
            features:Vec::new(),
            instructions:Vec::new(),
            test_fit: Some(global_params::params::MIN_FIT),
            cv_fit:Some(global_params::params::MIN_FIT),
            pos_missed: None,
            neg_missed: None,
        }
    }


    ////                 For Evaluating            ////


    pub fn create_compressed(&self) -> Program{
        let instr_i = self.get_effective_instrs_good(0);
        let instrs: Vec<Instruction> = instr_i.into_iter().map(|i| {self.instructions[i].clone()}).collect();
        Program{
            n_calc_regs: 0,
            features: Vec::new(),
            instructions: instrs,
            test_fit:None,
            cv_fit: None,
            pos_missed: None,
            neg_missed: None,
        }
    }


//    fn get_used_metabolites(&self, instrs: &Vec<Instruction>) -> Vec<u8> {
//        let mut used = HashSet::new();
//        for instr in instrs.iter(){
//
//        }
//    }



    ////                Getters           ////

    //return # of effective instructions
    pub fn get_effective_len(&self, return_reg_ind: u8) -> usize{
        self.get_effective_instrs_good(0).len()
    }


    //return # of effective instructions
    pub fn get_abs_len(&self) -> usize{
        self.instructions.len()
    }



    pub fn get_effective_instrs_good(&self, return_reg_ind: u8) -> Vec<usize>{
        if self.get_abs_len() == 0 {
            return Vec::new()
        }

        let mut eff_instrs = HashSet::new();

        for end_i in self.get_quit_set(0) {
            self.get_effective_instrs_from_fixed_final_point(0, end_i)
                .into_iter()
                .for_each(|x| {eff_instrs.insert(x);})
        }
        let mut eff_instrs: Vec<usize> = eff_instrs.drain().collect();
        eff_instrs.sort();
//        println!("final set is {:?}", &eff_instrs);
        eff_instrs
    }


    pub fn get_effective_instrs_from_fixed_final_point(&self, return_reg_ind: u8, end_i: usize) -> HashSet<usize>{

        let mut eff_regs = HashSet::new();
        let mut eff_instrs = HashSet::new();

        let mut last_eff = false;
        eff_regs.insert(return_reg_ind);
        eff_instrs.insert(end_i);


        for (i, instr) in self.instructions.iter().enumerate().rev(){
            if i > end_i {continue;}
            let follows_branch = i >= 1 && self.instructions[i-1].is_branch();

            match ops::get_type(instr) {

                InstructionType::Value => {
                    if eff_regs.contains(&instr.dest) {
                        if !follows_branch {
                            eff_regs.remove(&instr.dest);
                        }
                        eff_regs.insert(instr.src1);
                        eff_regs.insert(instr.src2);
                        eff_instrs.insert(i);
                        last_eff = true;
                    }
                    else {
                        last_eff = false;
                    }
                },

                InstructionType::Skip => {
                    if last_eff { // becuase branch only ever skips one.
                        eff_regs.insert(instr.src1);
                        eff_regs.insert(instr.src2);
                        eff_instrs.insert(i);
                    }
                },

                InstructionType::Terminate => {
                    last_eff = i == end_i; //asssume only effective is end_i
                },

                InstructionType::NoOp => {
                    last_eff = false //never effective
                },

            }
        }
//        println!("from end i {} set is {:?}", end_i, &eff_instrs);
        eff_instrs
    }



    fn get_exit_index(&self, return_reg_ind: u8) -> usize{

        let mut last_br = false;
        let mut first_uc_quit_index = 0;

        for instr in self.instructions.iter() {
            let op_type = ops::get_type(instr);
            if let InstructionType::Skip = op_type{
                last_br = true;
            }
            else {
                if let InstructionType::Terminate = op_type{
                    if !last_br {  //unconditional quit
                        break;
                    }
                }
                last_br = false;
            }
            first_uc_quit_index += 1;
        }
        for (i, instr) in self.instructions.iter().enumerate().rev(){
            if i <= first_uc_quit_index && instr.dest == return_reg_ind {
                return i
            }
        }
        0 // no assignemt to return reg before uc quit
    }


    //return index of all possible final instruction indexs upto and including prog termination point
    fn get_quit_set(&self, return_reg_ind: u8) -> HashSet<usize>{
        let last_ind = self.get_exit_index(return_reg_ind);
        let mut quit_set = HashSet::new();
        quit_set.insert(last_ind);

        for  (i, instr) in self.instructions.iter().enumerate() {
            if i == last_ind {
                break;
            }
            if let InstructionType::Terminate =  ops::get_type(instr){
                quit_set.insert(i);
            }
        }
        quit_set
    }



    pub fn get_percent_branch(&self, return_reg_ind: u8) -> f32{
        let mut eff_regs = HashSet::new();
        let mut last_eff = false;
        eff_regs.insert(return_reg_ind);

        let mut eff_instr_count = 0.0;
        let mut branch_count = 0.0;

        for (i, instr) in self.instructions.iter().enumerate().rev(){
            if instr.is_branch() {
                if last_eff { // becuase branch only ever skips one.
                    eff_instr_count += 1.0;
                    branch_count += 1.0;
                }
            }
                else {
                    if eff_regs.contains(&instr.dest) {
                        eff_regs.remove(&instr.dest);
                        eff_regs.insert(instr.src1);
                        eff_regs.insert(instr.src2);
                        last_eff = true;
                        eff_instr_count += 1.0;
                    }
                        else {
                            last_eff = false;
                        }
                }

        }
        branch_count/eff_instr_count
    }

//    // like effective, but also will always include instructions after
//    // branch statements. The output of only running these instrcutions should
//    // be identical to running all instructions.
//    pub fn get_important_instrs(&self, return_reg_ind: u8) -> Vec<usize>{
//        let mut eff_regs = HashSet::new();
//        let mut eff_instrs = HashSet::new();
//        eff_regs.insert(return_reg_ind);
//        for (i, instr) in self.instructions.iter().enumerate().rev(){
//            if eff_regs.contains(&instr.dest) {
//                eff_regs.insert(instr.src1);
//                eff_regs.insert(instr.src2);
//                eff_instrs.insert(i);
//            }
//            if instr.is_branch() && i < self.instructions.len()-1{ //dont do for last instr;
//                eff_instrs.insert(i+1);
//            }
//        }
//        let mut eff_instrs: Vec<usize>  = eff_instrs.into_iter().collect();
//        eff_instrs.sort();
//        eff_instrs
//    }


    pub fn get_n_effective_feats(&self, return_reg_ind: u8) -> usize{
        let mut eff_regs = HashSet::new();

        for instr_i in self.get_effective_instrs_good(0){
            let instr = self.instructions[instr_i];
            if instr.op == 6 {
                eff_regs.insert(instr.src1);
            }
            else {
                eff_regs.insert(instr.src1);
                eff_regs.insert(instr.src2);
            }
        }

        eff_regs
            .into_iter()
            .fold(0, |acc, x| if x >= (global_params::params::MAX_REGS - self.features.len()) as u8 {acc+1} else {acc})
    }

    pub fn get_effective_feats(&self, return_reg_ind: u8) -> HashSet<u8>{
        let mut eff_regs = HashSet::new();

        for instr_i in self.get_effective_instrs_good(0){
            let instr = self.instructions[instr_i];
//            if instr.op == 6 {    <- not sure why this was here, old op?
//                eff_regs.insert(instr.src1);
//            }
//                else {
//                    eff_regs.insert(instr.src1);
//                    eff_regs.insert(instr.src2);
//                }
            eff_regs.insert(instr.src1);
            eff_regs.insert(instr.src2);

        }
//        println!("second ind={} len={} regs{:?}", return_reg_ind, eff_regs.len(), &eff_regs);
        eff_regs.retain(|&x|  x >= (global_params::params::MAX_REGS - self.features.len()) as u8);
//        println!("thrid ind={} len={}", return_reg_ind, eff_regs.len());
        eff_regs.iter().map(|x| super::reg_2_feat(&self.features, x)).collect()
    }


    pub fn get_n_effective_comp_regs(&self, return_reg_ind: u8) -> usize{
        let mut eff_regs = HashSet::new();
        eff_regs.insert(return_reg_ind);
//        println!("prog abs len {}", self.get_abs_len());
//        println!("prog eff len {}\n", self.get_effective_len(0));
        for instr_i in self.get_effective_instrs_good(0){
            let instr = self.instructions[instr_i];
            if instr.op == 6 {
                eff_regs.insert(instr.src1);
            }
                else {
                    eff_regs.insert(instr.src1);
                    eff_regs.insert(instr.src2);
                }
        }
        eff_regs.into_iter().fold(0, |acc, x| if x < self.n_calc_regs {acc+1} else {acc})
    }



    ////                 For Logging            ////

    pub fn string_instr(&self, instr: &Instruction) -> String{

        let src1 =
            match self.get_global_feat_number(instr.src1) {
                Some(num) => format!("{}", metabolites::get_metabolite_by_ind(num)),
                None => format!("${}",instr.src1),
            };

        let src2 =
            match self.get_global_feat_number(instr.src2) {
                Some(num) => format!("{}", metabolites::get_metabolite_by_ind(num)),
                None => format!("${}",instr.src2),
            };

        ops::formatted_string(instr, &src1, &src2)
    }

    pub fn string_instr_better(&self, instr: &Instruction, used: &mut Vec<u8>) -> String{

        let src1 =
            match self.get_global_feat_number(instr.src1) {
                Some(num) => format!("{}", metabolites::get_metabolite_by_ind(num)),
                None => {
                    if used.contains(&instr.src1){
                        format!("${}",instr.src1)
                    }
                    else {
                        format!("{}",super::registers::PROG_REG[instr.src1 as usize])
                    }
                },
            };

        let src2 =
            match self.get_global_feat_number(instr.src2) {
                Some(num) => format!("{}", metabolites::get_metabolite_by_ind(num)),
                None => {
                    if used.contains(&instr.src2){
                        format!("${}",instr.src2)
                    }
                        else {
                            format!("{}",super::registers::PROG_REG[instr.src2 as usize])
                        }
                },
            };
        used.push(instr.dest);

        ops::formatted_string(instr, &src1, &src2)
    }


    pub fn feat_str(&self)->String{
        self.features.iter().fold(String::new(),
                     |mut acc, &x| {acc.push_str(dataMgmt::metabolites::get_metabolite_by_ind(x as usize)); acc.push_str("\t"); acc} )
    }


    pub fn print_self_words(&self){
        println!("{}", self.n_calc_regs);
        println!("{}", self.feat_str());
        for instr in self.instructions.iter(){
             println!("{}",self.string_instr(instr));
        }
    }


    pub fn write_header(&self, f: &mut File) {
        f.write(b"# Score on training data: ");
        f.write(self.test_fit.unwrap().to_string().as_bytes());
        f.write(b"\n");
        f.write(b"# Score on validation data: ");
        f.write(self.cv_fit.unwrap().to_string().as_bytes());
        f.write(b"\n");
        f.write(b"# Len: ");
        f.write(self.instructions.len().to_string().as_bytes());
        f.write(b"\n");
        f.write(b"# Eff Len: ");
        f.write(self.get_effective_len(0).to_string().as_bytes());
        f.write(b"\n");
        f.write(b"# Number of eff feats: ");
        f.write(self.get_n_effective_feats(0).to_string().as_bytes());
        f.write(b"\n");
        f.write(b"# Number of eff comp regs: ");
        f.write(self.get_n_effective_comp_regs(0).to_string().as_bytes());
        f.write(b"\n*");

        f.write(self.n_calc_regs.to_string().as_bytes());
        f.write(b"\n*");
        let feat_str = self.feat_str();
        f.write(feat_str.as_bytes());
    }

    pub fn write_self_words(&self, f: &mut File){
        f.write(b"\n\n## All instructions ##\n");
        for instr in self.instructions.iter(){
            let instr_str = self.string_instr(instr);
            f.write(instr_str.as_bytes());
            f.write(b"\n");
        }
        f.write(b"\n");
    }


    pub fn write_effective_self_words(&self, f: &mut File){
        let mut used_srcs = Vec::new();
        f.write(b"## Effective instructions ## \n");
        for instr_i in self.get_effective_instrs_good(0){
            let instr = self.instructions[instr_i];
            let instr_str = self.string_instr_better(&instr, &mut used_srcs);
            f.write(instr_str.as_bytes());
            f.write(b"\n");
        }
        f.write(b"\n");
    }

    pub fn write_effective_self_words_old(&self, f: &mut File){
        f.write(b"## Effective instructions ## \n");
        for instr_i in self.get_effective_instrs_good(0){
            let instr = self.instructions[instr_i];
            let instr_str = self.string_instr(&instr);
            f.write(instr_str.as_bytes());
            f.write(b"\n");
        }
        f.write(b"\n");
    }

    pub fn is_feature(&self, reg_num: u8) -> bool{
        reg_num >= self.n_calc_regs
    }

    //returns none if not feature
    pub fn get_global_feat_number(&self, reg_num: u8) -> Option<usize>{
        if self.is_feature(reg_num){
            let local_feat_i = global_params::params::MAX_REGS - reg_num as usize - 1; //0..n_feats
            let global_feat_i = self.features[local_feat_i];
            Some(global_feat_i as usize)
        }
        else {
            None
        }
    }

}










