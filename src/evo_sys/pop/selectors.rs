use super::maps::ResultMap;
use evo_sys::prog::prog::Program;

impl ResultMap{

    pub fn get_loc(&self, prog: &Program) -> (usize, usize) {
        match self.config.select_cell_method {
            0 => self.comp_len(prog),
            1 => self.eff_comp_eff_len(prog),
            _ => panic!("Invalid get location method!! \n{:?}", self.config),
        }
    }

    fn comp_len(&self, prog: &Program)  -> (usize, usize){
        (prog.n_calc_regs as usize, prog.get_abs_len() as usize)
    }

    fn eff_comp_eff_len(&self, prog: &Program)  -> (usize, usize){
        (prog.get_n_effective_comp_regs(0) as usize, prog.get_effective_len(0) as usize)
    }
}