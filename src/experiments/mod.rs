pub mod experiments;
pub mod mgmt;



#[derive(Debug)]
pub struct FiveFoldMultiTrial{
    pub select_cell_method: u8,
    pub compare_prog_method: u8,
    pub initial_pop: u32,
    pub total_evals: u64,
    pub n_iter: u32,
    pub out_folder: String,
    pub comment: String,
}


#[derive(Debug)]
pub struct Manager{
    pub select_cell_methods: Vec<u8>,
    pub compare_prog_methods: Vec<u8>,
    pub initial_pop: u32,
    pub total_evals: u64,
    pub n_iter: u32,
    pub out_folder: String,
    pub comment: String,
}


#[derive(Debug)]
pub struct Manager2{
    pub methods: Vec<(u8,u8)>,
    pub initial_pop: u32,
    pub total_evals: u64,
    pub n_iter: u32,
    pub out_folder: String,
    pub comment: String,
}