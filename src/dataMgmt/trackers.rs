use evo_sys::Program;

pub fn get_abs_geno_len(p: &Program) -> f32{
    p.get_abs_len() as f32
}

pub fn get_eff_geno_len(p: &Program) -> f32{
    p.get_effective_len(0) as f32
}

pub fn get_eff_feats(p: &Program) -> f32{
    p.get_n_effective_feats(0) as f32
}