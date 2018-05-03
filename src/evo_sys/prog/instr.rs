use evo_sys::prog::prog::get_src;
use evo_sys::prog::prog::Program;
use params;
use rand;
use rand::{Rng, seq, thread_rng, ThreadRng};
use rand::distributions::Range;
use rand::distributions::Sample;

#[derive(Copy, Clone, Debug)]
pub struct Instruction{
    pub dest: u8,
    pub op: u8,
    pub src1: u8,
    pub src2: u8,
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

    pub fn is_branch(&self)->bool{ //should be fixed to aviod manual update if more branches are added
        return self.op == 6 || self.op == 7
    }
}

