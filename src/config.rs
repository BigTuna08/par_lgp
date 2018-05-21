use std::fs::File;
use std::io::prelude::*;
use {CoreConfig, PopInfo, MapInfo, GenPopInfo, Runner, ConfigFile, Mode, ProgDefaults, ThreadDefaults};
use std;
use experiments::experiments;


use std::collections::HashMap;

pub struct ConfigManager{
    info: HashMap<String, Vec<String>>
}

impl ConfigManager{
    pub fn new(loc: &str) -> ConfigManager{
        let mut info = HashMap::new();
        println!("opening {}", loc);

        let mut f = File::open(loc).expect("error oping file!");
        let mut c = String::new();
        f.read_to_string(&mut c);

        for line in c.lines(){

            let mut parts = line.split_whitespace();
            let first = parts.next();
            match first {
                None => (),
                Some(text) if text.starts_with("//") => (), // comment -> do nothing
                Some(text) => {
                    info.insert(String::from(text), get_string_vect(&mut parts ));
                },
                _ => (),
            }
        }

        ConfigManager{
            info
        }
    }


    pub fn get_single_value<T: std::str::FromStr>(&self, key: &str) -> T {
        match self.info.get(key) {
            Some(val_vec) =>{
                match val_vec.len() {
                    0 => panic!("No values given with key={}", key),
                    _ => match val_vec[0].parse::<T>() {
                        Ok(val) => val,
                        Err(e) => panic!("error parsing configs with key={} ", key),
                    }
                }
            },
            None => panic!("not in config file! key={}", key),
        }
    }


    pub fn get_value_vec<T: std::str::FromStr>(&self, key: &str) -> Vec<T> {
        let mut list = Vec::new();

        match self.info.get(key) {
            Some(val_vec) =>{
                for val in val_vec{
                    match val.parse::<T>() {
                        Ok(val) => list.push(val),
                        Err(e) => panic!("error parsing configs with key={} ", key),
                    }

                }
            },
            None => panic!("not in config file! key={}", key),
        }
        list
    }
}




impl Runner{

    pub fn new(loc: &str) -> Runner{
        let config = process_experiment_config(loc);
        let mode = config.mode.clone();
        Runner{
            config,
            mode,
            mutate_i: 0,
            compare_i: 0,
            started: false,
            vec_1_i: 0,
            vec_2_i: 0,
            vec_3_i: 0,
        }
    }


    pub fn run_all_configs(&mut self){
        while let Some(config) = self.next_config(){
            experiments::multi_trial_five_fold_tracking(config);
        }
    }


    pub fn next_config(&mut self) -> Option<CoreConfig>{
        if !self.incr_inds(){
            return None
        }
        let data_file = self.config.data_file.clone();
        let compare_prog_method = self.config.compare_methods[self.compare_i];
        let mutate_method = self.config.mutate_methods[self.mutate_i];
        let n_iterations = self.config.n_iterations;

        let pop_config = match self.mode {
            Mode::Map => {
                PopInfo::Map(MapInfo {
                    select_cell_method: self.config.map_methods[self.vec_3_i],
                    initial_pop: self.config.inital_pop_size[self.vec_2_i],
                    n_evals:  self.config.n_evals[self.vec_1_i],
                })
            },
            Mode::Gen => {
                PopInfo::Gen(GenPopInfo {
                    tourn_size: self.config.tourn_sizes[self.vec_3_i],
                    total_gens: self.config.total_gens[self.vec_2_i],
                    random_gens:  self.config.random_gens[self.vec_1_i],
                })
            },
        };

        let out_folder =  match self.mode {//folder name is roughly mutate_compare_total_initial_other
            Mode::Map => format!("{}/{}_{}_{}_{}_{}",
                                 self.config.out_folder,
                                 self.config.mutate_methods[self.mutate_i],
                                 self.config.compare_methods[self.compare_i],
                                 self.config.n_evals[self.vec_1_i],
                                 self.config.inital_pop_size[self.vec_2_i],
                                 self.config.map_methods[self.vec_3_i],),

            Mode::Gen => format!("{}/{}_{}_{}_{}_{}",
                                 self.config.out_folder,
                                 self.config.mutate_methods[self.mutate_i],
                                 self.config.compare_methods[self.compare_i],
                                 self.config.random_gens[self.vec_1_i],
                                 self.config.total_gens[self.vec_2_i],
                                 self.config.tourn_sizes[self.vec_3_i],)
        };

        Some(CoreConfig{
            out_folder, data_file, compare_prog_method, mutate_method, pop_config, n_iterations,})
    }


    pub fn print_dry_run(&mut self){
        let mut i =1;
        while let Some(config) = self.next_config(){
            println!("config #{} is {:?}", i, &config);
            i += 1;
        }
    }

    fn incr_inds(&mut self) -> bool{// true means continue
        if !self.started{
            self.started = true;
            return true
        }

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
            Mode::Map => self.n_evals.len(),
            Mode::Gen => self.total_gens.len()
        }
    }

    pub fn v2_len(&self)-> usize{
        match self.mode {
            Mode::Map => self.inital_pop_size.len(),
            Mode::Gen => self.random_gens.len()
        }
    }

    pub fn v3_len(&self)-> usize{
        match self.mode {
            Mode::Map => self.map_methods.len(),
            Mode::Gen => self.tourn_sizes.len()
        }
    }
}


pub fn process_thread_defaults(loc: &str) -> ThreadDefaults {
    let info = ConfigManager::new(loc);
    ThreadDefaults{
        n_worker_threads: info.get_single_value("N_WORKER_THREADS:"),
        worker_queue_size: info.get_single_value("WORKER_QUEUE_SIZE:"),
        cap: info.get_single_value("WORKER_QUEUE_SIZE:"),
    }
}


pub fn process_prog_defaults(loc: &str) -> ProgDefaults {
    let info = ConfigManager::new(loc);
    ProgDefaults{
        INITIAL_INSTR_MIN: info.get_single_value("INITIAL_INSTR_MIN:"),
        INITIAL_INSTR_MAX: info.get_single_value("INITIAL_INSTR_MAX:"),
        INITIAL_CALC_REG_MIN: info.get_single_value("INITIAL_CALC_REG_MIN:"),
        INITIAL_CALC_REG_MAX: info.get_single_value("INITIAL_CALC_REG_MAX:"),
        INITIAL_N_OPS_MIN: info.get_single_value("INITIAL_N_OPS_MIN:"),
        INITIAL_N_OPS_MAX: info.get_single_value("INITIAL_N_OPS_MAX:"),
        INITIAL_FEAT_MIN: info.get_single_value("INITIAL_FEAT_MIN:"),
        INITIAL_FEAT_MAX: info.get_single_value("INITIAL_FEAT_MAX:"),
    }
}


pub fn get_log_freq(loc: &str) -> u32 {
    let info = ConfigManager::new("configs/experiment.txt");
    info.get_single_value("LOG_FREQ:")
}


fn process_experiment_config(loc: &str) -> ConfigFile{
    let info = ConfigManager::new(loc);

    ConfigFile {
        mode: str_to_mode(&info.get_single_value::<String>("MODE:")) ,
        data_file: info.get_single_value("DATA_FILE:"),
        out_folder: format!("results/{}", info.get_single_value::<String>("OUT_FOLDER:")),
        n_iterations: info.get_single_value("N_ITERATIONS:"),
        mutate_methods: info.get_value_vec("MUTATION_METHODS:"),
        compare_methods: info.get_value_vec("COMPARE_METHODS:"),

        n_evals: info.get_value_vec("N_EVALS:"),
        inital_pop_size: info.get_value_vec("INITIAL_POP_SIZES:"),
        map_methods: info.get_value_vec("MAP_METHODS:"),

        total_gens: info.get_value_vec("TOTAL_GENS:"),
        random_gens: info.get_value_vec("INIT_GENS:"),
        tourn_sizes: info.get_value_vec("TOURN_SIZE:"),
    }
}


fn str_to_mode(s: &str)->Mode{
    if s.eq_ignore_ascii_case("MAP"){
        return Mode::Map
    }
    else if s.eq_ignore_ascii_case("GEN"){
        return Mode::Gen
    }
    panic!("Error reading mode!!")
}


fn get_string_vect(parts: &mut std::str::SplitWhitespace) -> Vec<String>{
    let mut list = Vec::new();
    while let Some(text) = parts.next() {
        if text.starts_with("//") {
            break;
        }
        list.push(String::from(text));
    }
    list
}