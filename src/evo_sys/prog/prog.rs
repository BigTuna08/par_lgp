//extern crate rand;

use dataMgmt;
use params;
use rand;
use rand::{Rng, seq, thread_rng, ThreadRng};
use rand::distributions::Range;
use rand::distributions::Sample;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use super::instr::Instruction;
use super::ops;


#[derive(Debug)]
pub struct Program{
    pub n_calc_regs: u8,
    pub features: Vec<u8>,
    pub instructions: Vec<Instruction>,
    pub test_fit: Option<f32>,
    pub cv_fit: Option<f32>,
}




impl Program{

    pub fn new_default_range() -> Program{
        Program::new_random_range(1, 40,
                                  1, 40,
                                  params::params::N_OPS, params::params::N_OPS+1,
                                  1, 156)
    }


    pub fn new_random_range(instr_min: usize, instr_max: usize, calc_regs_min: u8,
                            calc_reg_max: u8, ops_min: u8, ops_max: u8, feats_min: u8, feats_max: u8) -> Program {

        let mut rng = thread_rng();

        Program::new_random(rng.gen_range(instr_min, instr_max ),
                                rng.gen_range(calc_regs_min, calc_reg_max),
                                    rng.gen_range(ops_min, ops_max),
                                    rng.gen_range(feats_min, feats_max))

    }


    pub fn new_random(n_instr: usize, n_calc_regs: u8, n_ops: u8, n_feats: u8) -> Program{
        let mut rng = thread_rng();

        let features = seq::sample_iter(&mut rng, 0..params::dataset::N_FEATURES, n_feats as usize).unwrap();
        let mut instructions = Vec::with_capacity(n_instr);

        for _ in 0..n_instr {
            instructions.push(Instruction::new_random(n_calc_regs, n_feats, n_ops, &mut rng));
        }

        Program{ n_calc_regs, features, instructions, test_fit:None, cv_fit:None}
    }


    pub fn execute_instructions(&self, mut regs: [f32; params::params::MAX_REGS]) ->f32{
        let mut skip_count = 0u8; // used to implement branches

        for instr in self.instructions.iter() {
            if skip_count > 0 {
                skip_count -= 1;
                continue;
            }
//            println!("instr: {:?}", &instr);
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


    pub fn string_instr(&self, instr: &Instruction) -> String{
        let n_feats = self.features.len() as u8;
        let src1 =
            if instr.src1 >= (params::params::MAX_REGS  as u8- n_feats) {
                let d = params::params::MAX_REGS - instr.src1 as usize -1; //0..n_feats
                let fest_num = self.features[d];
                format!("{}",&dataMgmt::headers::DATA_HEADERS[fest_num as usize])
            }else {
                format!("${}",instr.src1)
            };

        let src2 =
            if instr.src2 >= (params::params::MAX_REGS as u8 - n_feats) {
                let d = params::params::MAX_REGS - instr.src2 as usize - 1; //0..n_feats
                let fest_num = self.features[d];
                format!("{}",&dataMgmt::headers::DATA_HEADERS[fest_num as usize])
            }else {
                format!("${}",instr.src2)
            };
        format!("${}\t=\t{}\t{}\t{}", instr.dest, ops::OPS_NAMES[instr.op as usize], src1, src2)
    }

    pub fn feat_str(&self)->String{
        self.features.iter().fold(String::new(),
                     |mut acc, &x| {acc.push_str(&dataMgmt::headers::DATA_HEADERS[x as usize]); acc.push_str("\t"); acc} )
    }


    pub fn print_self_words(&self, feat_names: &Vec<String>){
        println!("{}", self.n_calc_regs);
        println!("{}", self.feat_str());
        for instr in self.instructions.iter(){
             println!("{}",self.string_instr(instr));
        }
    }

    pub fn write_self_words(&self, f: &mut File, feat_names: &Vec<String>){

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




    pub fn new_default() -> Program{
        Program::new_random(10, 6, params::params::N_OPS, 3)
    }




    //return a new empty program
    pub fn new_empty() -> Program{
        Program{n_calc_regs:0, features:Vec::new(), instructions:Vec::new(), test_fit:None, cv_fit:None}
    }


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
        let mut count = 0;
        eff_regs.insert(return_reg_ind);
        for (i, instr) in self.instructions.iter().enumerate().rev(){
            if eff_regs.contains(&instr.dest) {
                eff_regs.insert(instr.src1);
                eff_regs.insert(instr.src2);
                count += 1;
                eff_instrs.push(i);
            }
        }
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
        eff_regs.into_iter().fold(0, |acc, x| if x >= (params::params::MAX_REGS - self.features.len()) as u8 {acc+1} else {acc})
    }

    pub fn get_effective_feats(&self, return_reg_ind: u8) -> HashSet<u8>{
        let mut eff_regs = HashSet::new();
        eff_regs.insert(return_reg_ind);
        for instr in self.instructions.iter().rev(){
            if eff_regs.contains(&instr.dest) {
                eff_regs.insert(instr.src1);
                eff_regs.insert(instr.src2);
            }
        }
        eff_regs.retain(|&x|  x >= (params::params::MAX_REGS - self.features.len()) as u8);
        eff_regs.map
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
    // more methods below (private)
}





impl Program{

    pub fn test_mutate_copy(&self) -> Program{

        let n = rand::thread_rng().gen_range(0, 25);
//        println!("in test mut copy with n= {}\n prog is {:?}", n, &self);
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
            if rand::thread_rng().gen_weighted_bool(params::evolution::MUT_INSTR_COPY_RATE){
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
            if rng.gen_weighted_bool(params::evolution::INSTR_INSERT_RATE) {
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

            if !rng.gen_weighted_bool(params::evolution::INSTR_DEL_RATE) {
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
        let mut rng = rand::thread_rng();

        let mut instructions = Vec::with_capacity(self.instructions.len());

        for instr in self.instructions.iter() {
            instructions.push(instr.clone());
        }

        Program{n_calc_regs, features, instructions, test_fit:None, cv_fit:None}
    }

    //very simple now, just rm instr if it has deleted reg
    pub fn del_comp_copy(&self) -> Program{
        if self.n_calc_regs == 1 {
            return Program::new_random(self.instructions.len(), 5, params::params::N_OPS, self.features.len() as u8)
        }
        let features = self.features.clone();
        let n_calc_regs = self.n_calc_regs -1;
        let mut rng = rand::thread_rng();

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

        if self.features.len() == params::dataset::N_FEATURES as usize{ //just do micro mutation
            return self.mut_instr_copy()
        }

        let mut new_feat = rng.gen_range(0, params::dataset::N_FEATURES);
        let mut tries = 0;
        while features.contains(&new_feat) {
            new_feat =  rng.gen_range(0, params::dataset::N_FEATURES);
            tries += 1;
            if tries > params::params::DUPLICATE_TIME_OUT { panic!("Error getting non dupicate!, {:?}", &self)}
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

        let mut rng = rand::thread_rng();
        let mut features = self.features.clone();
        features.pop();
        let removed_reg = (params::params::MAX_REGS - 1- features.len()) as u8;

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
//        if self.features.len() == 1 {
//            return Program::new_random(self.instructions.len(), self.n_calc_regs, params::params::N_OPS, 5)
//        }
        if self.features.len() == params::dataset::N_FEATURES as usize { //just do micro mutation
            return self.mut_instr_copy()
        }

        let mut rng = rand::thread_rng();
        let mut features = self.features.clone();

        let mut new_feat = rng.gen_range(0, params::dataset::N_FEATURES);
        let mut tries = 0;
        while features.contains(&new_feat) {
            new_feat =  rng.gen_range(0, params::dataset::N_FEATURES);
            tries += 1;
            if tries > params::params::DUPLICATE_TIME_OUT { panic!("Error getting non dupicate!, {:?}", &self)}
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


impl Program{
    pub fn rand_instr(&self, rng: &mut ThreadRng) -> Instruction{
        Instruction{
            dest: Program::rand_dest(self, rng),
            op: Program::rand_op(self, rng),
            src1: Program::rand_src(self, rng),
            src2: Program::rand_dest(self, rng),
        }
    }

    pub fn rand_dest(&self, rng: &mut ThreadRng) -> u8{
        rng.gen_range(0, self.n_calc_regs)
    }

    pub fn rand_dest_exclude(&self, rng: &mut ThreadRng, exclude: u8) -> u8 {
        if self.n_calc_regs == 1 {return 0}

        let mut n = rng.gen_range(0, self.n_calc_regs);
        let mut tries = 0;
        while n == exclude {
            n = rng.gen_range(0, self.n_calc_regs);
            tries += 1;
            if tries > params::params::DUPLICATE_TIME_OUT { panic!("Error getting non dupicate!, {:?}", &self)}
        }
        n
    }

    pub fn rand_src(&self, rng: &mut ThreadRng) -> u8{
        get_src(self.n_calc_regs, self.features.len() as u8, rng)
    }

    pub fn rand_src_exclude(&self, rng: &mut ThreadRng, exclude: u8) -> u8 {
        let mut n = get_src(self.n_calc_regs, self.features.len() as u8, rng);
        let mut tries = 0;
        while n == exclude {
            n = get_src(self.n_calc_regs, self.features.len() as u8, rng);
            tries += 1;
            if tries > params::params::DUPLICATE_TIME_OUT { panic!("Error getting non dupicate!")}
        }
        n
    }

    pub fn rand_op(&self, rng: &mut ThreadRng) -> u8{
        rng.gen_range(0, params::params::N_OPS)
    }

    pub fn rand_op_exclude(&self, rng: &mut ThreadRng, exclude: u8) -> u8 {
        let mut tries = 0;
        let mut n = rng.gen_range(0, params::params::N_OPS);
        while n == exclude {
            n = rng.gen_range(0, params::params::N_OPS);
            tries += 1;
            if tries > params::params::DUPLICATE_TIME_OUT { panic!("Error getting non dupicate!")}
        }
        n
    }
}


//returns random number in [0,n_small)U[MAX_REGS-n_big, MAX_REGS)
pub fn get_src(n_small: u8, n_big: u8, rng: &mut ThreadRng) ->u8 {
    let mut val = rng.gen_range(0, n_small + n_big);

    if val >= n_small {val = params::params::MAX_REGS as u8 - val + n_small-1 }

    val
}


pub fn reg_2_feat(feat_list: &Vec<u8>, reg: u8) -> u8{
    let feat_i = params::params::MAX_REGS - reg as usize;
    feat_list[feat_i].clone()
}
