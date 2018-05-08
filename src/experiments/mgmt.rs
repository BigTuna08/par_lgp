use params;
use std::fs::File;
use std::io::Write;
use std::fs::create_dir_all;
use experiments::experiments;
use evo_sys::PopConfig;
use super::{FiveFoldMultiTrial, Manager, Manager2};


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

    pub fn get_map_config(&self) -> PopConfig {
        PopConfig {
            select_cell_method: self.select_cell_method,
            compare_prog_method: self.compare_prog_method,
            initial_pop: self.initial_pop,
            total_evals: self.total_evals,
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




impl Manager2{

    pub fn new(args: Vec<String>) -> Manager2{

        let out_folder = format!("results/{}/raw", args[1]);

        match create_dir_all(&out_folder) {
            Ok(_) =>{


                let mut methods = Vec::new();
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

                            i += 1;


                            let new_arg2 = &args[i];
                            let new_arg2 = new_arg2.parse::<u8>();
                            match new_arg2 {
                                Ok(new_arg2) =>{
                                    methods.push((new_arg, new_arg2));
                                    i += 1;
                                }
                                Err(e) => panic!("need select and compare method!! err is {:?}", e),
                            }

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


                if methods.is_empty(){
                    panic!("no methods given !!");
                }



                Manager2 { methods, initial_pop, total_evals, out_folder, n_iter, comment}
            }
            Err(e) => panic!("Problem creating out dir! {:?}\n Err is {:?}", &out_folder, e)
        }
    }

}