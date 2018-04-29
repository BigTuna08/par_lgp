use evo_sys;


#[derive(Debug)]
pub struct FiveFoldMultiTrial{
    pub eval_code: usize,
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
        let eval_code = arg_iter.next().unwrap().clone().parse::<usize>().unwrap();
        let initial_pop = arg_iter.next().unwrap().clone().parse::<u32>().unwrap();
        let total_evals = arg_iter.next().unwrap().clone().parse::<u64>().unwrap();
        let n_iter = arg_iter.next().unwrap().clone().parse::<u32>().unwrap();
        let out_folder = arg_iter.next().unwrap().clone();
        let comment = arg_iter.next().unwrap().clone();


        FiveFoldMultiTrial {eval_code, initial_pop, total_evals, out_folder, n_iter, comment}
    }
}

//args: <initial_pop> <total_evals> <out_folder> <n_iter> <comment> <list of eval codes>



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

