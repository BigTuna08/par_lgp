use evo_sys::prog::get_src;
use evo_sys::{Program, Instruction, InstructionResult, InstructionType};

use params as global_params;
use rand::{Rng, ThreadRng};
use rand::distributions::Range;
use rand::distributions::Sample;



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
            if tries > global_params::params::DUPLICATE_TIME_OUT { panic!("Error getting non dupicate!, {:?}", &self)}
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
            if tries > global_params::params::DUPLICATE_TIME_OUT { panic!("Error getting non dupicate!")}
        }
        n
    }

    pub fn rand_op(&self, rng: &mut ThreadRng) -> u8{
        rng.gen_range(0, global_params::params::N_OPS)
    }

    pub fn rand_op_exclude(&self, rng: &mut ThreadRng, exclude: u8) -> u8 {
        let mut tries = 0;
        let mut n = rng.gen_range(0, global_params::params::N_OPS);
        while n == exclude {
            n = rng.gen_range(0, global_params::params::N_OPS);
            tries += 1;
            if tries > global_params::params::DUPLICATE_TIME_OUT { panic!("Error getting non dupicate!")}
        }
        n
    }
}



impl Instruction{
    pub fn new_random(n_calc_regs: u8, n_feats: u8, n_ops:u8, rng: &mut ThreadRng) -> Instruction{
        let mut op_range = Range::new(0, n_ops);
        let mut dest_rng = Range::new(0, n_calc_regs);

        let dest = dest_rng.sample(rng);
        let op = op_range.sample(rng);
        let src1 = get_src(n_calc_regs, n_feats, rng);
        let src2 = get_src(n_calc_regs, n_feats, rng);

        Instruction{dest, op, src1, src2}
    }


    //this should be rewritten so it doesnt take prog ref, may just some info about prog in a struct
    pub fn mutate_copy(&self, prog: &Program, rng: &mut ThreadRng) -> Instruction{

        let &Instruction{ mut dest, mut op, mut src1, mut src2} = self;

        match rng.gen_range(0, 4) {
            0 => dest = prog.rand_dest_exclude(rng, dest),
            1 => op = prog.rand_op_exclude(rng, op),
            2 => src1 = prog.rand_src_exclude(rng, src1),
            3 => src2 = prog.rand_src_exclude(rng, src2),
            _ => panic!("Should never be here!")
        }

        Instruction{dest, op, src1, src2}
    }

    pub fn contains(&self, x: u8) -> bool {
        self.dest == x || self.op == x || self.src1 == x || self.src2 == x
    }

    pub fn contains_dest(&self, x: u8) -> bool {
        self.dest == x
    }

    pub fn contains_op(&self, x: u8) -> bool {
        self.op == x
    }

    pub fn contains_src(&self, x: u8) -> bool {
        self.src1 == x || self.src2 == x
    }

    pub fn contains_reg(&self, x: u8) -> bool {
        self.dest == x || self.src1 == x || self.src2 == x
    }


    pub fn is_branch(&self)->bool{
        if let InstructionType::Skip = super::ops::get_type(self) {true}
        else {false}
    }

}


