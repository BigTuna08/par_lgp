use super::maps::ResultMap;
use evo_sys::prog::prog::Program;

impl ResultMap{

    pub fn get_loc(&self, prog: &Program) -> (usize, usize) {
        match self.config.select_cell_method {
            0 => self.comp_len(prog),
            1 => self.eff_comp_eff_len(prog),
            2 => self.eff_comp_eff_len_3t(prog),
            3 => self.eff_comp_eff_feat(prog),
            4 => self.eff_comp_eff_feat_3t(prog),
            5 => self.eff_len_eff_feat(prog),
            6 => self.eff_comp_eff_feat_3t(prog),
            _ => panic!("Invalid get location method!! \n{:?}", self.config),
        }
    }

    fn comp_len(&self, prog: &Program)  -> (usize, usize){
        (prog.n_calc_regs as usize, prog.get_abs_len() as usize)
    }

    fn eff_comp_eff_len(&self, prog: &Program)  -> (usize, usize){
        (prog.get_n_effective_comp_regs(0) as usize, prog.get_effective_len(0) as usize)
    }

    fn eff_comp_eff_len_3t(&self, prog: &Program)  -> (usize, usize){
        let x = (prog.get_n_effective_comp_regs(0) / 3) as usize;
        let y = ( prog.get_effective_len(0) / 3) as usize;
        (x, y)
    }

    fn eff_comp_eff_feat(&self, prog: &Program)  -> (usize, usize){
        let x = (prog.get_n_effective_comp_regs(0)) as usize;
        let y = ( prog.get_n_effective_feats(0)) as usize;
        (x, y)
    }

    fn eff_comp_eff_feat_3t(&self, prog: &Program)  -> (usize, usize){
        let x = (prog.get_n_effective_comp_regs(0) / 3) as usize;
        let y = ( prog.get_n_effective_feats(0) / 3) as usize;
        (x, y)
    }

    fn eff_len_eff_feat(&self, prog: &Program)  -> (usize, usize){
        let x = ( prog.get_effective_len(0) ) as usize;
        let y = ( prog.get_n_effective_feats(0)) as usize;
        (x, y)
    }

    fn eff_len_eff_feat_3t(&self, prog: &Program)  -> (usize, usize){
        let x = ( prog.get_effective_len(0) / 3) as usize;
        let y = ( prog.get_n_effective_feats(0) / 3) as usize;
        (x, y)
    }


}