use super::super::{ResultMap, Program};
use params::params::{MAP_ROWS, MAP_COLS};

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
        let row = ( prog.get_percent_branch(0)*MAP_ROWS as f32) as usize;
        let col = prog.get_effective_len(0);
        (row, col)
    }


    fn e_feat_br(&self, prog: &Program)  -> (usize, usize){
        let row = ( prog.get_percent_branch(0)*MAP_ROWS as f32) as usize;
        let col = prog.get_n_effective_feats(0) as usize;
        (row, col)
    }

    fn e_len_feat_br(&self, prog: &Program)  -> (usize, usize){
        let feats = (prog.get_n_effective_feats(0) as f32).powi(2);
        let len =  (prog.get_effective_len(0) as f32).powi(2);

        let col = ( feats / (feats + len))*1.25;// const 5/4 makes range 0-1
        let col = (col*MAP_COLS as f32) as usize;
        let row = ( prog.get_percent_branch(0)*MAP_ROWS as f32) as usize;

        (row, col)
    }



}

