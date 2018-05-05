use super::super::{ResultMap, Program};

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


}