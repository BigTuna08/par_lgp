use std::fs::File;
use std::io::prelude::*;
use {CoreConfig, PopInfo, MapInfo, GenPopInfo, Runner, ConfigFile, Mode};
use std;
use experiments::experiments;

pub fn get_runner(loc: &str) -> Runner{
    let config = process_config(loc);
    let mode = config.mode.clone();
    Runner{
        config,
        mode,
        mutate_i: 0,
        compare_i: 0,
        vec_1_i: 0,
        vec_2_i: 0,
        vec_3_i: 0,
    }
}


impl Runner{

    pub fn run_all_configs(&mut self){
        while let Some(config) = self.next_config(){
            experiments::multi_trial_five_fold_tracking(config);
        }
    }

    pub fn next_config(&mut self) -> Option<CoreConfig>{
        println!("starting next config ind are {} {} {} {} {}", self.mutate_i, self.compare_i, self.vec_1_i, self.vec_2_i, self.vec_3_i);

        let data_file = self.config.data_file.clone();
        let compare_prog_method = self.config.compare_methods[self.compare_i];
        let mutate_method = self.config.mutate_methods[self.mutate_i];
        let n_iterations = self.config.n_iterations;

        let pop_config = match self.mode {
            Mode::Map => {
                PopInfo::Map(MapInfo {
                    select_cell_method: self.config.get_map_methods(self.vec_3_i).unwrap(),
                    initial_pop: self.config.get_inital_pop_size(self.vec_2_i).unwrap(),
                    n_evals: self.config.get_n_eval(self.vec_1_i).unwrap(),
                })
            },
            Mode::Gen => {
                PopInfo::Gen(GenPopInfo {
                    tourn_size: self.config.get_tourn_sizes(self.vec_3_i).unwrap(),
                    total_gens: self.config.get_total_gens(self.vec_2_i).unwrap(),
                    random_gens: self.config.get_random_gens(self.vec_1_i).unwrap(),
                })
            },
        };
        println!("mid: ind are {} {} {} {} {}", self.mutate_i, self.compare_i, self.vec_1_i, self.vec_2_i, self.vec_3_i);
        println!("also config is {:?}", self.config );

        let out_folder =  match self.mode {//folder name is roughly mutate_compare_total_initial_other
            Mode::Map => format!("{}/{}_{}_{}_{}_{}",
                                 self.config.out_folder,
                                 self.config.mutate_methods[self.mutate_i],
                                 self.config.compare_methods[self.compare_i],
                                 self.config.get_n_eval(self.vec_1_i).unwrap(),
                                 self.config.get_inital_pop_size(self.vec_2_i).unwrap(),
                                 self.config.get_map_methods(self.vec_3_i).unwrap()),
            Mode::Gen => format!("{}/{}_{}_{}_{}_{}",
                                 self.config.out_folder,
                                 self.config.mutate_methods[self.mutate_i],
                                 self.config.compare_methods[self.compare_i],
                                 self.config.get_total_gens(self.vec_1_i).unwrap(),
                                 self.config.get_random_gens(self.vec_2_i).unwrap(),
                                 self.config.get_tourn_sizes(self.vec_3_i).unwrap()),
        };
        println!("got out f: ind are {} {} {} {} {}", self.mutate_i, self.compare_i, self.vec_1_i, self.vec_2_i, self.vec_3_i);

        if self.incr_inds(){
            Some(CoreConfig{
                out_folder, data_file, compare_prog_method, mutate_method, pop_config, n_iterations,
            })
        }
        else { None }

    }

    pub fn print_dry_run(&mut self){
        let mut i =1;
        while let Some(config) = self.next_config(){
            println!("config #{} is {:?}", i, &config);
            i += 1;
        }
    }

    fn incr_inds(&mut self) -> bool{// true means continue
        self.vec_3_i += 1;

        if self.vec_3_i >= self.config.v3_len(){
            self.vec_3_i = 0;
            self.vec_2_i += 1;
        }

        if self.vec_2_i >= self.config.v2_len() {
            self.vec_2_i = 0;
            self.vec_1_i += 1;
        }

        if self.vec_1_i >= self.config.v1_len() {
            self.vec_1_i = 0;
            self.compare_i += 1;
        }

        if self.compare_i >= self.config.compare_methods.len() {
            self.compare_i = 0;
            self.mutate_i += 1;
        }

        self.mutate_i < self.config.mutate_methods.len()
    }
}



impl ConfigFile{
    pub fn v1_len(&self)-> usize{
        match self.mode {
            Mode::Map => match self.n_evals {
                Some(ref x) => x.len(),
                None => panic!("Cannot get len becuase donesnt exist!!"),
            }
            Mode::Gen => match self.total_gens {
                Some(ref x) => x.len(),
                None => panic!("Cannot get len becuase donesnt exist!!"),
            }
        }
    }

    pub fn v2_len(&self)-> usize{
        match self.mode {
            Mode::Map => match self.inital_pop_size {
                Some(ref x) => x.len(),
                None => panic!("Cannot get len becuase donesnt exist!!"),
            }
            Mode::Gen => match self.random_gens {
                Some(ref x) => x.len(),
                None => panic!("Cannot get len becuase donesnt exist!!"),
            }
        }
    }

    pub fn v3_len(&self)-> usize{
        match self.mode {
            Mode::Map => match self.map_methods {
                Some(ref x) => x.len(),
                None => panic!("Cannot get len becuase donesnt exist!!"),
            }
            Mode::Gen => match self.tourn_sizes {
                Some(ref x) => x.len(),
                None => panic!("Cannot get len becuase donesnt exist!!"),
            }
        }
    }

    pub fn get_n_eval(&self, i: usize)-> Option<u64>{
        match self.n_evals {
            Some(ref n_evals) => Some(n_evals[i]),
            None => None,
        }
    }

    pub fn get_inital_pop_size(&self, i: usize)-> Option<u32>{
        match self.inital_pop_size {
            Some(ref inital_pop_size) => Some(inital_pop_size[i]),
            None => None,
        }
    }

    pub fn get_map_methods(&self, i: usize)-> Option<u8>{
        match self.map_methods {
            Some(ref map_methods) => Some(map_methods[i]),
            None => None,
        }
    }

    pub fn get_total_gens(&self, i: usize)-> Option<u32>{
        match self.total_gens {
            Some(ref total_gens) => Some(total_gens[i]),
            None => None,
        }
    }
    pub fn get_random_gens(&self, i: usize)-> Option<u32>{
        match self.random_gens {
            Some(ref random_gens) => Some(random_gens[i]),
            None => None,
        }
    }

    pub fn get_tourn_sizes(&self, i: usize)-> Option<u16>{
        match self.tourn_sizes {
            Some(ref tourn_sizes) => Some(tourn_sizes[i]),
            None => None,
        }
    }

}

fn process_config(loc: &str) -> ConfigFile{

    let mut mode = None;
    let mut out_folder = None;
    let mut data_file = None;
    let mut n_iterations = None;
    let mut mutate_methods = None;
    let mut compare_methods = None;

    let mut n_evals = None;
    let mut inital_pop_size = None;
    let mut map_methods = None;

    let mut total_gens = None;
    let mut init_gens = None;
    let mut tourn_sizes = None;


    let mut f = File::open(loc).expect("error oping file!");
    let mut c = String::new();
    f.read_to_string(&mut c);
    for line in c.lines(){
        let mut parts = line.split_whitespace();
        let first = parts.next();
        match first {
            None => (),
            Some(text) if text.eq_ignore_ascii_case("MODE:") => {
                mode = Some(get_mode(&mut parts))
            },
            Some(text) if text.eq_ignore_ascii_case("OUT_FOLDER:") => {
                out_folder = Some(get_next_string(&mut parts))
            },
            Some(text) if text.eq_ignore_ascii_case("N_ITERATIONS:") => {
                n_iterations = Some(get_next_u32(&mut parts))
            },
            Some(text) if text.eq_ignore_ascii_case("DATA_FILE:") => {
                data_file = Some(get_next_string(&mut parts))
            },
            Some(text) if text.eq_ignore_ascii_case("MUTATION_METHODS:") => {
                mutate_methods = Some(get_option_list(&mut parts))
            },
            Some(text) if text.eq_ignore_ascii_case("COMPARE_METHOD:") => {
                compare_methods = Some(get_option_list(&mut parts))
            },
            Some(text) if text.eq_ignore_ascii_case("N_EVALS:") => {
                n_evals = Some(get_option_list(&mut parts))
            },
            Some(text) if text.eq_ignore_ascii_case("INITIAL_POP_SIZES:") => {
                inital_pop_size = Some(get_option_list(&mut parts))
            }
            Some(text) if text.eq_ignore_ascii_case("MAP_METHODS:") => {
                map_methods = Some(get_option_list(&mut parts))
            }
            Some(text) if text.eq_ignore_ascii_case("TOTAL_GENS:") => {
                total_gens = Some(get_option_list(&mut parts))
            },
            Some(text) if text.eq_ignore_ascii_case("INIT_GENS:") => {
                init_gens = Some(get_option_list(&mut parts))
            },
            Some(text) if text.eq_ignore_ascii_case("TOURN_SIZE:") => {
                tourn_sizes = Some(get_option_list(&mut parts))
            },
            _ => (),
        }
    }


    let mode = mode.unwrap();
    let out_folder = out_folder.unwrap();
    let data_file = data_file.unwrap();
    let n_iterations = n_iterations.unwrap();
    let mutate_methods = mutate_methods.unwrap();
    let compare_methods = compare_methods.unwrap();


    ConfigFile{
        mode,
        data_file,
        out_folder,
        n_iterations,
        mutate_methods,
        compare_methods,

        n_evals,
        inital_pop_size,
        map_methods,

        total_gens,
        random_gens: init_gens,
        tourn_sizes,
    }
}




fn get_mode(parts: &mut std::str::SplitWhitespace) -> Mode{
    if let Some(p) = parts.next() {
        if p.eq_ignore_ascii_case("MAP"){
            return Mode::Map
        }
            else if p.eq_ignore_ascii_case("GEN"){
                return Mode::Gen
            }
    }
    panic!("Error reading mode!!")
}


fn get_next_string(parts: &mut std::str::SplitWhitespace) -> String {
    if let Some(out_folder) = parts.next() {
        return String::from(out_folder)
    }
    panic!("Error geting string!")
}

fn get_next_u32(parts: &mut std::str::SplitWhitespace) -> u32{
    if let Some(text) = parts.next() {
        return match text.parse::<u32>() {
            Ok(val) => val,
            Err(e) => panic!("Error reading value for number of iterations. Value was {}. \
            \n Err was {:?}", text, e),
        }
    }
    panic!("error getting u32!!")
}

fn get_option_list<T: std::str::FromStr>(parts: &mut std::str::SplitWhitespace) -> Vec<T>{
    let mut list = Vec::new();
    while let Some(text) = parts.next() {
        match text.parse::<T>() {
            Ok(val) => list.push(val),
            Err(_) => break, //list is done
        };
    }
    if list.len() == 0 {
        panic!("Error getting list!")
    }
    list
}
