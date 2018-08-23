use super::super::{ResultMap, Program};
use params::params::{MAP_ROWS, MAP_COLS};
use super::VarPenConfig;

impl ResultMap{

    pub fn select_cell(&self, prog: &Program) -> (usize, usize) {
        match self.config.select_cell_method{
            0 => self.eff_comp_eff_feat(prog, 1),
            1 => self.eff_comp_eff_len(prog, 1),
            2 => self.eff_len_eff_feat(prog, 1),
            3 =>  self.comp_eff_feat(prog, 1),
            4 =>  self.abs_len_eff_feat(prog, 1),
            5 =>  self.eff_len_eff_feat_improved(prog),
            6 =>  self.comp_feat(prog, 1),
            7 => self.e_comp_feat_len(prog),
            8 => self.e_comp_feat_len2(prog),
            9 => self.e_comp_feat_len3(prog),
            10 => self.e_len_br(prog),
            11 => self.e_feat_br(prog),
            12 => self.e_len_feat_br(prog),
//            13 => self.newone(prog, 5, 10, VarPenConfig::new(-5.0,
//                                                         5.0,
//                                                         3.0,
//                                                         self.config.initial_pop as u64,
//                                                         self.config.initial_pop as u64,
//                                                         self.config.n_evals)),

            14 => self.newone(prog, 5, 10, VarPenConfig::new(0.0, //no varible, protects all!!
                                                             1.0,
                                                             3.0,
                                                             self.config.n_evals,
                                                             self.config.n_evals,
                                                             self.config.n_evals)),
            15 => self.pos_neg(prog),
            16 => self.pos_neg_len(prog),
            17 => self.metabolite_len(prog),

            _ => panic!("Invalid get location method!! \n{:?}", self.config),
        }
    }

    fn eff_comp_eff_feat(&self, prog: &Program, scale: usize)  -> (usize, usize){
        let row = prog.get_n_effective_comp_regs(0) as usize / scale;
        let col = prog.get_n_effective_feats(0) as usize / scale;
        (row, col)
    }

    fn eff_comp_eff_len(&self, prog: &Program, scale: usize)  -> (usize, usize){
        let row = prog.get_n_effective_comp_regs(0) as usize / scale;
        let col = prog.get_effective_len(0) as usize / scale;
        (row, col)
    }

    fn eff_len_eff_feat(&self, prog: &Program, scale: usize)  -> (usize, usize){
        let row = prog.get_effective_len(0)  as usize / scale;
        let col = prog.get_n_effective_feats(0) as usize / scale;
        (row, col)
    }

    fn comp_eff_feat(&self, prog: &Program, scale: usize)  -> (usize, usize){
        let row = prog.n_calc_regs as usize / scale;
        let col = prog.get_n_effective_feats(0) as usize / scale;
        (row, col)
    }

    fn abs_len_eff_feat(&self, prog: &Program, scale: usize)  -> (usize, usize){
        let row = prog.get_abs_len() / scale;
        let col = prog.get_n_effective_feats(0) as usize / scale;
        (row, col)
    }

    // this fills in dead area of map that happens because progs with few eff instr cannot have
    // many eff feats
    fn eff_len_eff_feat_improved(&self, prog: &Program)  -> (usize, usize){
        let feats = prog.get_n_effective_feats(0) as usize;

        let row = prog.get_effective_len(0)  - (feats+1)/2 ;
        let col = feats;
        (row, col)
    }

    fn comp_feat(&self, prog: &Program, scale: usize)  -> (usize, usize){
        let row = prog.n_calc_regs as usize / scale;
        let col = prog.features.len() / scale;
        (row, col)
    }

    fn e_comp_feat_len(&self, prog: &Program)  -> (usize, usize){
        let feats = (prog.get_n_effective_feats(0) as f32).powi(2);
        let comp = (prog.get_n_effective_comp_regs(0) as f32).powi(2);
        let len =  (prog.get_effective_len(0) as f32).powi(2);
        let row = ( comp/ (comp + feats));
        let col = (len / (len + feats));

        let row = (row*MAP_ROWS as f32) as usize;
        let col = (col*MAP_COLS as f32) as usize;
        (row, col)
    }

    fn e_comp_feat_len2(&self, prog: &Program)  -> (usize, usize){
        let feats = (prog.get_n_effective_feats(0) as f32).powi(2);
        let comp = (prog.get_n_effective_comp_regs(0) as f32).powi(2);
        let len =  (prog.get_effective_len(0) as f32).powi(2);

        let row = ( comp/ (comp + feats + len))*1.11; // const ~ 10/9 makes range 0-1
        let col = ( feats / (comp + feats + len))*1.25;// const 5/4 makes range 0-1

        let row = (row*MAP_ROWS as f32) as usize;
        let col = (col*MAP_COLS as f32) as usize;
        (row, col)
    }


    fn e_comp_feat_len3(&self, prog: &Program)  -> (usize, usize){
        let feats = (prog.get_n_effective_feats(0) as f32).powi(2);
        let comp = (prog.get_n_effective_comp_regs(0) as f32).powi(2);
        let len =  (prog.get_effective_len(0) as f32).powi(2);

        let row = ( comp/ (comp + len))*1.11; // const ~ 10/9 makes range 0-1
        let col = ( feats / (feats + len))*1.25;// const 5/4 makes range 0-1

        let row = (row*MAP_ROWS as f32) as usize;
        let col = (col*MAP_COLS as f32) as usize;
        (row, col)
    }

    fn e_len_br(&self, prog: &Program)  -> (usize, usize){
        let col = prog.get_effective_len(0);


        if col > 0{ //dont need progs with eff len=0
            let br = prog.get_percent_branch(0);
            let row = ( br*MAP_ROWS as f32) as usize;
//            println!("row={} col={} br={}", row, col, br);
            (row, col-1)
        }
        else {
            (10000, 10000) //out of bounds, used random numbers
        }


    }


    fn e_feat_br(&self, prog: &Program)  -> (usize, usize){
        let row = ( prog.get_percent_branch(0)*MAP_ROWS as f32) as usize;
        let col = prog.get_n_effective_feats(0) as usize;


        let br = prog.get_percent_branch(0);
        let row = ( br*MAP_ROWS as f32) as usize;
//        println!("row={} col={} br={}", row, col, br);

        (row, col)
    }

    fn e_len_feat_br(&self, prog: &Program)  -> (usize, usize){
        let feats = (prog.get_n_effective_feats(0) as f32).powi(2);
        let len =  (prog.get_effective_len(0) as f32).powi(2);

        let col = ( feats / (feats + len))*1.25;// const 5/4 makes range 0-1
        let col = (col*MAP_COLS as f32) as usize;


        let br = prog.get_percent_branch(0);
        let row = ( br*MAP_ROWS as f32) as usize;
//        println!("row={} col={} br={}", row, col, br);
        (row, col)
    }

    fn newone(&self, prog: &Program, sq: usize, base_eff_len_max: usize, pen_config: VarPenConfig)  -> (usize, usize){
        let eff_len = prog.get_effective_len(0) as f32;

        let eff_len_step = base_eff_len_max as f32 + pen_config.penalty_at(self.recieved_count);

        let loc_el = (eff_len/eff_len_step) * sq as f32;
        let loc_el = loc_el as usize;


        let loc_br = prog.get_percent_branch(0) *sq as f32;
        let loc_br = loc_br as usize;


        let feats = prog.get_n_effective_feats(0) as f32;
        let comp =  prog.get_n_effective_comp_regs(0) as f32;
        let loc_fc =  (feats/(feats+comp)) * sq as f32;
        let loc_fc = loc_fc as usize;

        let intron = eff_len/(prog.get_abs_len() as f32);
        let loc_intron = intron * sq as f32;
        let loc_intron = loc_intron as usize;


        let row = loc_el*sq + loc_fc;
        let col = loc_br*sq + loc_intron;
        (row, col)
    }

    fn pos_neg(&self, prog: &Program) -> (usize, usize){
        match prog.pos_missed {
            Some(p) => (p as usize, prog.neg_missed.unwrap() as usize),
            None => (0,0), // before eval
        }

    }

    fn pos_neg_len(&self, prog: &Program) -> (usize, usize){
        match prog.pos_missed {
            Some(p) => {
                let f = p as f32 / ((prog.neg_missed.unwrap() + p) as f32) ;
                (prog.get_effective_len(0), (f*MAP_COLS as f32) as usize)
            },
            None => (0,0), // before eval
        }

    }

    fn metabolite_len(&self, prog: &Program) -> (usize, usize){
        const metabolite_inds: [u8; 5] = [2,140,25,0,22,];
        let prog_mets = prog.get_effective_feats(0);

        let mut y = 0;
        let mut y_inc = 1;
        for mi in metabolite_inds.iter() {
            if prog_mets.contains(mi){
                y += y_inc;
            }
            y_inc *= 2;
        }

        (prog.get_effective_len(0), y)
    }
}

