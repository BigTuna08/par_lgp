use params;
use std::fs::create_dir_all;
use experiments::config::FiveFoldMultiTrial;
use experiments::experiments;

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


impl Manager {
    pub fn run_all(&self) {
        for s in self.select_cell_methods.iter(){
            for c in self.compare_prog_methods.iter(){
                let config = FiveFoldMultiTrial{
                    select_cell_method:*s,
                    compare_prog_method:*c,
                    initial_pop: self.initial_pop,
                    total_evals: self.total_evals,
                    n_iter: self.n_iter,
                    comment: self.comment.clone(),
                    out_folder: self.out_folder.clone(),
                };
                experiments::multi_trial_five_fold_tracking(config);
            }
        }
    }
}



impl Manager{

    pub fn new(args: Vec<String>) -> Manager{

        let out_folder = format!("results/{}/raw", args[1]);

        match create_dir_all(&out_folder) {
            Ok(_) =>{
                let mut select_cell_methods = Vec::new();
                let mut compare_prog_methods = Vec::new();
                let mut initial_pop = params::defaults::DEFAULT_INITIAL_POP;
                let mut total_evals = params::defaults::DEFAULT_TOTAL_EVALS;
                let mut n_iter = params::defaults::DEFAULT_ITERS;
                let mut comment = String::from(params::defaults::DEFAULT_COMMENT);


                let mut i = 0;
                while i < args.len() {
                    let mut arg = &args[i];

                    if arg.eq_ignore_ascii_case("-s") {
                        i += 1;
                        while i < args.len() && !&args[i].starts_with("-") {
                            let new_arg = &args[i];
                            let new_arg = new_arg.parse::<u8>().unwrap();
                            select_cell_methods.push(new_arg);
                            i += 1;
                        }
                    }

                        else if arg.eq_ignore_ascii_case("-c"){
                            i += 1;
                            while i < args.len() && !&args[i].starts_with("-") {
                                let new_arg = &args[i];
                                let new_arg = new_arg.parse::<u8>().unwrap();
                                compare_prog_methods.push(new_arg);
                                i += 1;

                            }
                        }
                            else if arg.eq_ignore_ascii_case("-p"){
                                i += 1;
                                if i < args.len() && !&args[i].starts_with("-") {
                                    let new_arg = &args[i];
                                    initial_pop = new_arg.parse::<u32>().unwrap();
                                    i += 1;
                                }
                            }

                                else if arg.eq_ignore_ascii_case("-i"){
                                    i += 1;
                                    if i < args.len() && !&args[i].starts_with("-") {
                                        let new_arg = &args[i];
                                        n_iter = new_arg.parse::<u32>().unwrap();
                                        i += 1;
                                    }
                                }

                                else if arg.eq_ignore_ascii_case("-e"){
                                    i += 1;
                                    if i < args.len() && !&args[i].starts_with("-") {
                                        let new_arg = &args[i];
                                        total_evals = new_arg.parse::<u64>().unwrap();
                                        i += 1;
                                    }
                                }

                                    else if arg.eq_ignore_ascii_case("-m"){
                                        i += 1;
                                        if i < args.len() && !&args[i].starts_with("-") {
                                            comment =  args[i].clone();
                                            i += 1;
                                        }
                                    }

                                        else {
                                            i += 1;
                                        }
                }


                if select_cell_methods.is_empty(){
                    select_cell_methods.push(params::defaults::DEFAULT_SELECT_CELL);
                }
                if compare_prog_methods.is_empty(){
                    compare_prog_methods.push(params::defaults::DEFAULT_COMPARE_PROG);
                }


                Manager { select_cell_methods, compare_prog_methods, initial_pop, total_evals, out_folder, n_iter, comment}
            }
            Err(e) => panic!("Problem creating out dir! {:?}\n Err is {:?}", &out_folder, e)
        }
    }

}