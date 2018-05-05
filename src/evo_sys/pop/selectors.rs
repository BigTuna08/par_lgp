use super::maps::ResultMap;
use evo_sys::prog::prog::Program;

impl ResultMap{

    pub fn get_loc(&self, prog: &Program) -> (usize, usize) {
        match self.config.select_cell_method{
            0 => self.eff_comp_eff_feat(prog, 1),
            1 => self.eff_comp_eff_len(prog, 1),
            2 => self.eff_len_eff_feat(prog, 1),
            3 =>  self.comp_eff_feat(prog, 1),
            4 =>  self.abs_len_eff_feat(prog, 1),
//            5 =>  self.eff_comp_eff_feat(prog, 1),
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


}