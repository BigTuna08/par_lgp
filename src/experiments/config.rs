use evo_sys;


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


impl FiveFoldMultiTrial{


    pub fn new(args: Vec<String>) -> FiveFoldMultiTrial{
        let mut arg_iter = args.iter();
        arg_iter.next();
        let select_cell_method = arg_iter.next().unwrap().clone().parse::<u8>().unwrap();
        let compare_prog_method = arg_iter.next().unwrap().clone().parse::<u8>().unwrap();
        let initial_pop = arg_iter.next().unwrap().clone().parse::<u32>().unwrap();
        let total_evals = arg_iter.next().unwrap().clone().parse::<u64>().unwrap();
        let n_iter = arg_iter.next().unwrap().clone().parse::<u32>().unwrap();
        let out_folder = arg_iter.next().unwrap().clone();
        let comment = arg_iter.next().unwrap().clone();


        FiveFoldMultiTrial { select_cell_method, compare_prog_method, initial_pop, total_evals, out_folder, n_iter, comment}
    }

    pub fn new_default(out_folder: &str) -> FiveFoldMultiTrial{
        FiveFoldMultiTrial{
            select_cell_method: 0,
            compare_prog_method: 0,
            initial_pop: 10_000,
            total_evals: 100_000,
            n_iter: 5,
            out_folder: String::from(out_folder),
            comment: String::from("testing with default"),
        }
    }

    pub fn get_map_config(&self) -> MapConfig{
        MapConfig{
            select_cell_method: self.select_cell_method,
            compare_prog_method: self.compare_prog_method,
            initial_pop: self.initial_pop,
            total_evals: self.total_evals,
        }
    }
}


pub struct MapConfig{
    pub select_cell_method: u8,
    pub compare_prog_method: u8,
    pub initial_pop: u32,
    pub total_evals: u64,
}




#[derive(Debug)]
pub struct Config{
    pub initial_pop: u32,
    pub total_evals: u64,
    pub out_folder: String,
    pub n_iter: u32,
    pub comment: String,

    pub current_eval_code_i: Option<usize>,
    pub eval_codes: Vec<usize>,

//    pub eval_descs: Iterator<String>,
}




//args: <initial_pop> <total_evals> <out_folder> <n_iter> <comment> <list of eval codes>
impl Config{
    pub fn new(args: Vec<String>) -> Config{
        let mut arg_iter = args.iter();
        arg_iter.next();
        let initial_pop = arg_iter.next().unwrap().clone().parse::<u32>().unwrap();
        let total_evals = arg_iter.next().unwrap().clone().parse::<u64>().unwrap();
        let out_folder = arg_iter.next().unwrap().clone();
        let n_iter = arg_iter.next().unwrap().clone().parse::<u32>().unwrap();
        let comment = arg_iter.next().unwrap().clone();

        let mut eval_codes: Vec<usize> = arg_iter.map(|x| x.parse::<usize>().unwrap()).collect();

        Config {initial_pop, total_evals, out_folder, n_iter, comment, current_eval_code_i:None, eval_codes}
    }

    //true-> continue, false -> quit
    pub fn next_eval_code(&mut self)->bool{
        match self.current_eval_code_i {
            Some(mut ec) => self.current_eval_code_i = Some(ec+1),
            None => self.current_eval_code_i = Some(0)
        }
        self.current_eval_code_i.unwrap() < self.eval_codes.len()
    }

    pub fn get_current_eval_code(&self)-> usize{
        self.eval_codes[self.current_eval_code_i.unwrap()]
    }

    pub fn get_current_eval_desc(&self)-> String{
        String::from(evo_sys::prog::eval::EVALS_DESC[self.get_current_eval_code()])
    }

}

