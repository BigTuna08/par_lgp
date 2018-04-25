use config::Config;
use dataMgmt::dataset::ValidationSet;
use dataMgmt::logger::GenoEval;
use dataMgmt::message::EvalResult;
//use std::collections::HashMap;
use indexmap::IndexMap;
use params;
use progSystem;
use progSystem::prog::Program;
use rand;
use rand::Rng;
use std;
use std::fs::File;
use std::io::Write;

pub enum PopEval<'a>{
    TestFit,
    CV,
    Geno(&'a GenoEval),
}


pub struct ResultMap{
    prog_map: IndexMap<(usize, usize),Program>,
}


impl ResultMap {
    pub fn new() -> ResultMap {
        ResultMap {
            prog_map: IndexMap::new(),
        }
    }

    pub fn get_test_fit(&self, inds: &(usize, usize)) -> f32 {
        match self.prog_map.get(inds) {
            Some(prog) => prog.test_fit.unwrap(),
            None => params::MIN_FIT,
        }
    }

    pub fn get_cv_fit(&self, inds: &(usize, usize))-> f32 {
        match self.prog_map.get(inds) {
            Some(prog) => prog.test_fit.unwrap(),
            None => params::MIN_FIT,
        }
    }

    pub fn put(&mut self, val: EvalResult, inds: &(usize, usize)) {
        self.prog_map.insert(*inds, val.genome);
    }


    //returns true only if made an improvement
    pub fn try_put(&mut self, new_entry: EvalResult) -> PutResult {
        let inds = &new_entry.map_location.unwrap();
        let new_fit = new_entry.genome.test_fit.unwrap();
        let old_fit = self.get_test_fit(inds);

        let result =
            if inds.0 >= params::MAP_ROWS || inds.1 >= params::MAP_COLS || new_fit < old_fit { PutResult::Failed }
            else if new_fit > old_fit { PutResult::Improvement }
            else if rand::thread_rng().gen_weighted_bool(params::REPLACE_EQ_FIT) { PutResult::Equal }
            else { PutResult::Failed }; //eq but not replaced

        match result {
            PutResult::Failed => (),
            _ => self.put(new_entry, inds),
        }
        result
    }

    //returns true only if made an improvement
    pub fn try_put_trial_based(&mut self, new_entry: EvalResult, trial_no: u64) -> PutResult {

        let inds = &new_entry.map_location.unwrap();

        let (new_fit, old_fit) =
            if (trial_no > 6_000_000 && trial_no < 9_000_000) ||
                (trial_no > 16_000_000 && trial_no < 19_000_000) {

                match self.prog_map.get(inds) {
                    Some(prog) => (get_adjusted_fit(&new_entry.genome, trial_no), get_adjusted_fit(&prog,trial_no)),
                    None => (get_adjusted_fit(&new_entry.genome, trial_no),params::MIN_FIT),
                }

            }else{
                (new_entry.genome.test_fit.unwrap(),self.get_test_fit(inds))
            };


        let result =
            if inds.0 >= params::MAP_ROWS || inds.1 >= params::MAP_COLS || new_fit < old_fit { PutResult::Failed }
            else if new_fit > old_fit { PutResult::Improvement }
            else if rand::thread_rng().gen_weighted_bool(params::REPLACE_EQ_FIT) { PutResult::Equal }
            else { PutResult::Failed }; //eq but not replaced

        match result {
            PutResult::Failed => (),
            _ => self.put(new_entry, inds),
        }
        result
    }


    //returns true only if made an improvement
    pub fn try_put_trial_based_config(&mut self, new_entry: EvalResult, trial_no: u64, config: &Config, method_code: u8) -> PutResult {

        let inds = &new_entry.map_location.unwrap();

        let new_fit = get_adjusted_fit_config_n(&new_entry.genome, trial_no, config, method_code);

        let old_fit =
                match self.prog_map.get(inds) {
                    Some(prog) => get_adjusted_fit_config_n(&prog, trial_no, config, method_code),
                    None => params::MIN_FIT,
                };

        let result =
            if inds.0 >= params::MAP_ROWS || inds.1 >= params::MAP_COLS || new_fit < old_fit { PutResult::Failed }
                else if new_fit > old_fit { PutResult::Improvement }
                    else if rand::thread_rng().gen_weighted_bool(params::REPLACE_EQ_FIT) { PutResult::Equal }
                        else { PutResult::Failed }; //eq but not replaced

        match result {
            PutResult::Failed => (),
            _ => self.put(new_entry, inds),
        }
        result
    }


    //pick random item from geno map and return random mutated copy
    pub fn get_simple_mutated_genome_rand(&self) -> Program {
        self.prog_map.get_index(rand::thread_rng().gen_range(0, self.prog_map.len())).unwrap().1.test_mutate_copy()
    }


    pub fn write_test_fits(&self, file_name: &str){
        let mut f = File::create(file_name).unwrap();
        for row_i in 0..params::MAP_ROWS {
            for col_i in 0..params::MAP_COLS {
                let fit = self.get_test_fit(&(row_i, col_i));
                f.write(fit.to_string().as_bytes());
                f.write(b"\t");
            }
            f.write(b"\n");
        }
    }




    pub fn write_cv_fits(&self, file_name: &str){
        let mut f = File::create(file_name).unwrap();
        for row_i in 0..params::MAP_ROWS {
            for col_i in 0..params::MAP_COLS {
                let fit = self.get_cv_fit(&(row_i, col_i));
                f.write(fit.to_string().as_bytes());
                f.write(b"\t");
            }
            f.write(b"\n");
        }
    }


    pub fn write_genos(&self, file_name: &str){
        let mut f = File::create(file_name).unwrap();
        for row_i in 0..params::MAP_ROWS {
            for col_i in 0..params::MAP_COLS {
                let geno = self.prog_map.get(&(row_i, col_i));
                if let Some(ref genome) = geno{
                    f.write(b"(");
                    f.write(row_i.to_string().as_bytes());
                    f.write(b",");
                    f.write(col_i.to_string().as_bytes());
                    f.write(b")");
                    f.write(b"\n");
                    genome.write_effective_self_words(&mut f);
                }
            }
        }
    }


    pub fn write_genos_abs_len(&self, file_name: &str){
        let mut f = File::create(file_name).unwrap();
        for row_i in 0..params::MAP_ROWS {
            for col_i in 0..params::MAP_COLS {

                if let Some(ref genome) = self.prog_map.get(&(row_i, col_i)){
                    f.write(genome.instructions.len().to_string().as_bytes());
                    f.write(b"\t");
                }
                else {
                    f.write(b"-1.0\t");
                }
            }
            f.write(b"\n");
        }

    }

    pub fn write_genos_eff_len(&self, file_name: &str){
        let mut f = File::create(file_name).unwrap();
        for row_i in 0..params::MAP_ROWS {
            for col_i in 0..params::MAP_COLS {

                if let Some(ref genome) = self.prog_map.get(&(row_i, col_i)){
                    f.write(genome.get_effective_len(0).to_string().as_bytes());
                    f.write(b"\t");
                }
                else {
                    f.write(b"-1.0\t");
                }
            }
            f.write(b"\n");
        }
    }


    fn get_sample_inds(&self) -> &(usize, usize) {
        let i = rand::thread_rng().gen_range(0, self.prog_map.len());
        self.prog_map.get_index(i).unwrap().0
    }

}


impl ResultMap{
//    pub fn get_test_stats(&self) -> MapStats{
//        let mut best = std::f32::MIN;
//        let mut worst = std::f32::MAX;
//        let mut ave = 0.0f64;
//        let mut count = 0.0;
//
//        for row_i in 0.. params::MAP_ROWS{
//            for col_i in 0.. params::MAP_COLS{
//                let fit = self.get_test_fit(&(row_i, col_i));
//
//                if fit > params::MIN_FIT{
//                    ave += fit as f64;
//                    count += 1.0;
//                    if fit > best {best=fit;}
//                    if fit < worst {worst=fit;}
//                }
//
//            }
//        }
//        ave = ave/count;
//
//        let mut vari = 0.0;
//        for row_i in 0.. params::MAP_ROWS{
//            for col_i in 0.. params::MAP_COLS{
//                let fit = self.get_test_fit(&(row_i, col_i));
//                if fit != params::MIN_FIT{
//                    vari += (fit as f64-ave).powi(2);
//                }
//            }
//        }
//        vari /= count;
//
//        MapStats {best, worst, ave, sd:vari.sqrt(), count: count as f32}
//    }

    pub fn has_prog(&self, inds: &(usize, usize))->bool{
        match self.prog_map.get(inds) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn get_map_stats(&self, eval: PopEval) -> MapStats{
        let mut best = std::f32::MIN;
        let mut worst = std::f32::MAX;
        let mut ave = 0.0f64;
        let mut count = 0.0;

        for row_i in 0.. params::MAP_ROWS{
            for col_i in 0.. params::MAP_COLS{

                if self.has_prog(&(row_i, col_i)){
                    let value = match eval {
                        PopEval::TestFit => self.get_test_fit(&(row_i, col_i)),
                        PopEval::CV => self.get_cv_fit(&(row_i, col_i)),
                        PopEval::Geno(eval) => eval(self.prog_map.get(&(row_i, col_i)).unwrap()),
                    };

                    ave += value as f64;
                    count += 1.0;
                    if value > best {best=value;}
                    if value < worst {worst=value;}
                }

            }
        }
        ave = ave/count;

        let mut vari = 0.0;
        for row_i in 0.. params::MAP_ROWS{
            for col_i in 0.. params::MAP_COLS{
                if self.has_prog(&(row_i, col_i)){
                    let value = match eval {
                        PopEval::TestFit => self.get_test_fit(&(row_i, col_i)),
                        PopEval::CV => self.get_cv_fit(&(row_i, col_i)),
                        PopEval::Geno(eval) => eval(self.prog_map.get(&(row_i, col_i)).unwrap()),
                    };
                    vari += (value as f64-ave).powi(2);
                }
            }
        }
        vari /= count;
        MapStats {best, worst, ave, sd:vari.sqrt(), count: count as f32}
    }

//
//    pub fn get_cv_stats(&self) -> MapStats {
//        let mut best = std::f32::MIN;
//        let mut worst = std::f32::MAX;
//        let mut ave = 0.0f64;
//        let mut count = 0.0;
//
//
//        for row_i in 0.. params::MAP_ROWS{
//            for col_i in 0.. params::MAP_COLS{
//                if let Some(genome) = self.prog_map.get(&(row_i, col_i)) {
//                    let fit = genome.cv_fit.unwrap();
//                    ave += fit as f64;
//                    count += 1.0;
//                    if fit > best {best=fit;}
//                    if fit < worst {worst=fit;}
//                }
//            }
//        }
//        ave = ave/count;
//
//        let mut vari = 0.0;
//        for row_i in 0.. params::MAP_ROWS{
//            for col_i in 0.. params::MAP_COLS{
//                if let Some(genome) =  self.prog_map.get(&(row_i, col_i)){
//                    let fit = genome.cv_fit.unwrap();
//                    vari += (fit as f64-ave).powi(2);
//                }
//            }
//        }
//        vari /= count;
//
//        MapStats {best, worst, ave, sd:vari.sqrt(), count:count as f32}
//    }


    pub fn update_cross_validation(&mut self, data: &ValidationSet) {
        for row_i in 0.. params::MAP_ROWS{
            for col_i in 0.. params::MAP_COLS{
                if let Some(genome) = self.prog_map.get_mut(&(row_i, col_i)) {
                    match genome.cv_fit {
                        Some(_) => (),
                        None =>  genome.cv_fit = Some(progSystem::eval::eval_program_cv(genome, &data)),
                    }
                }
            }
        }
    }


//    pub fn get_geno_stats(&self, eval: &GenoEval) -> MapStats {
//        let mut best = std::f32::MIN;
//        let mut worst = std::f32::MAX;
//        let mut ave = 0.0f64;
//        let mut count = 0.0;
//
//        for row_i in 0.. params::MAP_ROWS{
//            for col_i in 0.. params::MAP_COLS{
//                if let Some(ref genome) = self.prog_map.get(&(row_i, col_i)) { //does exist
//                    let score = eval(*genome);
//                    ave += score as f64;
//                    count += 1.0;
//                    if score > best {best=score;}
//                    if score < worst {worst=score;}
//                }
//
//            }
//        }
//        ave = ave/count;
//
//        let mut vari = 0.0;
//        for row_i in 0.. params::MAP_ROWS{
//            for col_i in 0.. params::MAP_COLS{
//                let fit = self.get_cv_fit(&(row_i, col_i));
//                if fit != params::MIN_FIT{
//                    vari += (fit as f64-ave).powi(2);
//                }
//            }
//        }
//        vari /= count;
//
//        MapStats {best, worst, ave, sd:vari.sqrt(), count:count as f32}
//    }
}

fn get_adjusted_fit(prog: &Program, trial_no: u64)->f32{
    let period = 2_000_000.0;

    let min = -1.5/params::TEST_DATA_SET_SIZE as f64;
    let max = 1.5/params::TEST_DATA_SET_SIZE as f64;

    let ampli = (max - min)/2.0;
    let mid = (max+min)/2.0;

    let value = (trial_no as f64)*2.0*std::f64::consts::PI/period;
    let mut penalty = mid + ampli*value.sin();
    penalty *= prog.get_effective_len(0) as f64;
    prog.test_fit.unwrap() - penalty as f32
}


fn get_adjusted_fit_config_n(prog: &Program, trial_no: u64, config: &Config, n: u8)->f32{
    match n {
        1 => get_adjusted_fit_config1(prog, trial_no, config),
        2 => get_adjusted_fit_config2(prog, trial_no, config),
        3 => get_adjusted_fit_config3(prog, trial_no, config),
        4 => get_adjusted_fit_config4(prog, trial_no, config),
        5 => get_adjusted_fit_config5(prog, trial_no, config),
        6 => get_adjusted_fit_config6(prog, trial_no, config),
        7 => get_adjusted_fit_config7(prog, trial_no, config),
        8 => get_adjusted_fit_config8(prog, trial_no, config),
        9 => get_adjusted_fit_config9(prog, trial_no, config),
        10 => get_adjusted_fit_config10(prog, trial_no, config),
        11 => get_adjusted_fit_config11(prog, trial_no, config),
        12 => get_adjusted_fit_config12(prog, trial_no, config),
        13 => get_adjusted_fit_config13(prog, trial_no, config),
        14 => get_adjusted_fit_config14(prog, trial_no, config),
        _ => panic!("invalid method code!!")
    }
}

fn get_adjusted_fit_config1(prog: &Program, trial_no: u64, config: &Config)->f32{

    let adj_start_trial = config.total_evals / 10;
    let adj_end_trial = adj_start_trial*9;

    if trial_no < adj_start_trial || trial_no > adj_end_trial{
        return prog.test_fit.unwrap()
    }

    let n_waves = 1.5;
    let min_penalty = -5.0/params::TEST_DATA_SET_SIZE as f64;
    let max_penalty = 5.0/params::TEST_DATA_SET_SIZE as f64;


    let period = ((adj_end_trial - adj_end_trial) as f64)/n_waves;
    let ampli = (max_penalty - min_penalty)/2.0;
    let mid = (max_penalty+min_penalty)/2.0;

    let value = ((trial_no-adj_start_trial) as f64)*2.0*std::f64::consts::PI/period;
    let mut penalty = mid + ampli*value.sin();
    penalty *= prog.get_effective_len(0) as f64;

    prog.test_fit.unwrap() - penalty as f32
}


fn get_adjusted_fit_config2(prog: &Program, trial_no: u64, config: &Config)->f32{

    let adj_start_trial = config.total_evals / 10;
    let adj_end_trial = adj_start_trial*9;

    if trial_no < adj_start_trial || trial_no > adj_end_trial{
        return prog.test_fit.unwrap()
    }

    let n_waves = 2.0;
    let min_penalty = -1.5/params::TEST_DATA_SET_SIZE as f64;
    let max_penalty = 1.5/params::TEST_DATA_SET_SIZE as f64;


    let period = ((adj_end_trial - adj_end_trial) as f64)/n_waves;
    let ampli = (max_penalty - min_penalty)/2.0;
    let mid = (max_penalty+min_penalty)/2.0;

    let value = ((trial_no-adj_start_trial) as f64)*2.0*std::f64::consts::PI/period;
    let mut penalty = mid + ampli*value.sin();
    penalty *= prog.get_effective_len(0) as f64;

    prog.test_fit.unwrap() - penalty as f32
}


fn get_adjusted_fit_config3(prog: &Program, trial_no: u64, config: &Config)->f32{

    let adj_start_trial = config.total_evals / 10;
    let adj_end_trial = adj_start_trial*9;

    if trial_no < adj_start_trial || trial_no > adj_end_trial{
        return prog.test_fit.unwrap()
    }

    let n_waves = 3.5;
    let min_penalty = -1.5/params::TEST_DATA_SET_SIZE as f64;
    let max_penalty = 1.5/params::TEST_DATA_SET_SIZE as f64;


    let period = ((adj_end_trial - adj_end_trial) as f64)/n_waves;
    let ampli = (max_penalty - min_penalty)/2.0;
    let mid = (max_penalty+min_penalty)/2.0;

    let value = ((trial_no-adj_start_trial) as f64)*2.0*std::f64::consts::PI/period;
    let mut penalty = mid + ampli*value.sin();
    penalty *= prog.get_effective_len(0) as f64;

    prog.test_fit.unwrap() - penalty as f32
}



fn get_adjusted_fit_config4(prog: &Program, trial_no: u64, config: &Config)->f32{

    let adj_start_trial = config.total_evals / 10;
    let adj_end_trial = adj_start_trial*9;

    if trial_no < adj_start_trial || trial_no > adj_end_trial{
        return prog.test_fit.unwrap()
    }

    let n_waves = 2.0;
    let min_penalty = -1.5/params::TEST_DATA_SET_SIZE as f64;
    let max_penalty = 1.5/params::TEST_DATA_SET_SIZE as f64;


    let period = ((adj_end_trial - adj_end_trial) as f64)/n_waves;
    let ampli = (max_penalty - min_penalty)/2.0;
    let mid = (max_penalty+min_penalty)/2.0;

    let value = ((trial_no-adj_start_trial) as f64)*2.0*std::f64::consts::PI/period;
    let mut penalty = mid + ampli*value.sin();
    penalty *= prog.get_effective_len(0) as f64;

    prog.test_fit.unwrap() + penalty as f32
}


fn get_adjusted_fit_config5(prog: &Program, trial_no: u64, config: &Config)->f32{

    let adj_start_trial = config.total_evals / 10;
    let adj_end_trial = adj_start_trial*9;

    if trial_no < adj_start_trial || trial_no > adj_end_trial{
        return prog.test_fit.unwrap()
    }

    let n_waves = 1.5;
    let min_penalty = 0.0/params::TEST_DATA_SET_SIZE as f64;
    let max_penalty = 3.0/params::TEST_DATA_SET_SIZE as f64;


    let period = ((adj_end_trial - adj_end_trial) as f64)/n_waves;
    let ampli = (max_penalty - min_penalty)/2.0;
    let mid = (max_penalty+min_penalty)/2.0;

    let value = ((trial_no-adj_start_trial) as f64)*2.0*std::f64::consts::PI/period;
    let mut penalty = mid + ampli*value.sin();
    penalty *= prog.get_effective_len(0) as f64;

    prog.test_fit.unwrap() - penalty as f32
}


fn get_adjusted_fit_config6(prog: &Program, trial_no: u64, config: &Config)->f32{

    let adj_start_trial = config.total_evals / 10;
    let adj_end_trial = adj_start_trial*9;

    if trial_no < adj_start_trial || trial_no > adj_end_trial{
        return prog.test_fit.unwrap()
    }

    let n_waves = 1.5;
    let min_penalty = -1.5/params::TEST_DATA_SET_SIZE as f64;
    let max_penalty =4.5 /params::TEST_DATA_SET_SIZE as f64;


    let period = ((adj_end_trial - adj_end_trial) as f64)/n_waves;
    let ampli = (max_penalty - min_penalty)/2.0;
    let mid = (max_penalty+min_penalty)/2.0;

    let value = ((trial_no-adj_start_trial) as f64)*2.0*std::f64::consts::PI/period;
    let mut penalty = mid + ampli*value.sin();
    penalty *= prog.get_effective_len(0) as f64;

    prog.test_fit.unwrap() - penalty as f32
}




//fixed pen
fn get_adjusted_fit_config7(prog: &Program, trial_no: u64, config: &Config)->f32{

    let adj_start_trial = config.total_evals / 10;
    let adj_end_trial = adj_start_trial*9;

    if trial_no < adj_start_trial || trial_no > adj_end_trial{
        return prog.test_fit.unwrap()
    }


    let mut penalty = 0.5/params::TEST_DATA_SET_SIZE as f64;
    penalty *= prog.get_effective_len(0) as f64;

    prog.test_fit.unwrap() - penalty as f32
}


//bigger fixed
fn get_adjusted_fit_config8(prog: &Program, trial_no: u64, config: &Config)->f32{

    let adj_start_trial = config.total_evals / 10;
    let adj_end_trial = adj_start_trial*9;

    if trial_no < adj_start_trial || trial_no > adj_end_trial{
        return prog.test_fit.unwrap()
    }


    let mut penalty = 1.5/params::TEST_DATA_SET_SIZE as f64;
    penalty *= prog.get_effective_len(0) as f64;

    prog.test_fit.unwrap() - penalty as f32
}


fn get_adjusted_fit_config9(prog: &Program, trial_no: u64, config: &Config)->f32{

    let adj_start_trial = config.total_evals / 10;
    let adj_end_trial = adj_start_trial*9;

    if trial_no < adj_start_trial || trial_no > adj_end_trial{
        return prog.test_fit.unwrap()
    }

    let n_waves = 3.5;
    let min_penalty = -1.5/params::TEST_DATA_SET_SIZE as f64;
    let max_penalty = 1.5/params::TEST_DATA_SET_SIZE as f64;


    let period = ((adj_end_trial - adj_end_trial) as f64)/n_waves;
    let ampli = (max_penalty - min_penalty)/2.0;
    let mid = (max_penalty+min_penalty)/2.0;

    let value = ((trial_no-adj_start_trial) as f64)*2.0*std::f64::consts::PI/period;
    let mut penalty = mid + ampli*value.sin();


    if penalty > 0.0{
        penalty *= prog.get_effective_len(0) as f64;
    }else{
        penalty *= prog.get_n_effective_feats(0) as f64;
    }


    prog.test_fit.unwrap() - penalty as f32
}



fn get_adjusted_fit_config10(prog: &Program, trial_no: u64, config: &Config)->f32{

    let adj_start_trial = config.total_evals / 10;
    let adj_end_trial = adj_start_trial*9;

    if trial_no < adj_start_trial || trial_no > adj_end_trial{
        return prog.test_fit.unwrap()
    }

    let n_waves_len = 3.5;
    let min_penalty__len = -1.5/params::TEST_DATA_SET_SIZE as f64;
    let max_penalty_len = 1.5/params::TEST_DATA_SET_SIZE as f64;


    let period_len = ((adj_end_trial - adj_end_trial) as f64)/n_waves_len;
    let ampli_len = (max_penalty_len - min_penalty__len)/2.0;
    let mid_len = (max_penalty_len+min_penalty__len)/2.0;

    let value_len = ((trial_no-adj_start_trial) as f64)*2.0*std::f64::consts::PI/period_len;
    let mut penalty_len = mid_len + ampli_len*value_len.sin();




    let n_waves = 3.5;
    let min_bonus = 0.0/params::TEST_DATA_SET_SIZE as f64;
    let max_bonus = 1.5/params::TEST_DATA_SET_SIZE as f64;


    let period = ((adj_end_trial - adj_end_trial) as f64)/n_waves;
    let ampli = (max_bonus - min_bonus)/2.0;
    let mid = (max_bonus+min_bonus)/2.0;

    let value = ((trial_no-adj_start_trial) as f64)*2.0*std::f64::consts::PI/period;
    let mut bonus = mid + ampli*value.sin();



    prog.test_fit.unwrap() + (bonus - penalty_len) as f32
}


fn get_adjusted_fit_config11(prog: &Program, trial_no: u64, config: &Config)->f32{

    let adj_start_trial = config.total_evals / 10;
    let adj_end_trial = adj_start_trial*9;

    if trial_no < adj_start_trial || trial_no > adj_end_trial{
        return prog.test_fit.unwrap()
    }

    let n_waves_len = 3.5;
    let min_penalty__len = -0.5/params::TEST_DATA_SET_SIZE as f64;
    let max_penalty_len = 2.5/params::TEST_DATA_SET_SIZE as f64;


    let period_len = ((adj_end_trial - adj_end_trial) as f64)/n_waves_len;
    let ampli_len = (max_penalty_len - min_penalty__len)/2.0;
    let mid_len = (max_penalty_len+min_penalty__len)/2.0;

    let value_len = ((trial_no-adj_start_trial) as f64)*2.0*std::f64::consts::PI/period_len;
    let mut penalty_len = mid_len + ampli_len*value_len.sin();




    let n_waves = 3.5;
    let min_bonus = 0.0/params::TEST_DATA_SET_SIZE as f64;
    let max_bonus = 4.5/params::TEST_DATA_SET_SIZE as f64;


    let period = ((adj_end_trial - adj_end_trial) as f64)/n_waves;
    let ampli = (max_bonus - min_bonus)/2.0;
    let mid = (max_bonus+min_bonus)/2.0;

    let value = ((trial_no-adj_start_trial) as f64)*2.0*std::f64::consts::PI/period;
    let mut bonus = mid + ampli*value.sin();



    prog.test_fit.unwrap() + (bonus - penalty_len) as f32
}


fn get_adjusted_fit_config12(prog: &Program, trial_no: u64, config: &Config)->f32{

    let adj_start_trial = config.total_evals / 10;
    let adj_end_trial = adj_start_trial*9;

    if trial_no < adj_start_trial || trial_no > adj_end_trial{
        return prog.test_fit.unwrap()
    }

    let n_waves_len = 3.5;
    let min_penalty__len = -0.25/params::TEST_DATA_SET_SIZE as f64;
    let max_penalty_len = 1.5/params::TEST_DATA_SET_SIZE as f64;


    let period_len = ((adj_end_trial - adj_end_trial) as f64)/n_waves_len;
    let ampli_len = (max_penalty_len - min_penalty__len)/2.0;
    let mid_len = (max_penalty_len+min_penalty__len)/2.0;

    let value_len = ((trial_no-adj_start_trial) as f64)*2.0*std::f64::consts::PI/period_len;
    let mut penalty_len = mid_len + ampli_len*value_len.sin();




    let n_waves = 3.5;
    let min_bonus = 0.0/params::TEST_DATA_SET_SIZE as f64;
    let max_bonus = 1.5/params::TEST_DATA_SET_SIZE as f64;


    let period = ((adj_end_trial - adj_end_trial) as f64)/n_waves;
    let ampli = (max_bonus - min_bonus)/2.0;
    let mid = (max_bonus+min_bonus)/2.0;

    let value = ((trial_no-adj_start_trial) as f64)*2.0*std::f64::consts::PI/period;
    let mut bonus = mid + ampli*value.sin();



    prog.test_fit.unwrap() + (bonus - penalty_len) as f32
}

fn get_adjusted_fit_config13(prog: &Program, trial_no: u64, config: &Config)->f32{

    let adj_start_trial = config.total_evals / 10;
    let adj_end_trial = adj_start_trial*9;

    if trial_no < adj_start_trial || trial_no > adj_end_trial{
        return prog.test_fit.unwrap()
    }

    let n_waves_len = 3.5;
    let min_penalty__len = -0.25/params::TEST_DATA_SET_SIZE as f64;
    let max_penalty_len = 1.5/params::TEST_DATA_SET_SIZE as f64;


    let period_len = ((adj_end_trial - adj_end_trial) as f64)/n_waves_len;
    let ampli_len = (max_penalty_len - min_penalty__len)/2.0;
    let mid_len = (max_penalty_len+min_penalty__len)/2.0;

    let value_len = ((trial_no-adj_start_trial) as f64)*2.0*std::f64::consts::PI/period_len;
    let mut penalty_len = mid_len + ampli_len*value_len.sin();




    let n_waves = 7.0;
    let min_bonus = 0.0/params::TEST_DATA_SET_SIZE as f64;
    let max_bonus = 1.5/params::TEST_DATA_SET_SIZE as f64;


    let period = ((adj_end_trial - adj_end_trial) as f64)/n_waves;
    let ampli = (max_bonus - min_bonus)/2.0;
    let mid = (max_bonus+min_bonus)/2.0;

    let value = ((trial_no-adj_start_trial) as f64)*2.0*std::f64::consts::PI/period;
    let mut bonus = mid + ampli*value.sin();



    prog.test_fit.unwrap() + (bonus - penalty_len) as f32
}

fn get_adjusted_fit_config14(prog: &Program, trial_no: u64, config: &Config)->f32{

    let adj_start_trial = config.total_evals / 10;
    let adj_end_trial = adj_start_trial*9;

    if trial_no < adj_start_trial || trial_no > adj_end_trial{
        return prog.test_fit.unwrap()
    }

    let n_waves_len = 3.5;
    let min_penalty__len = -0.25/params::TEST_DATA_SET_SIZE as f64;
    let max_penalty_len = 1.5/params::TEST_DATA_SET_SIZE as f64;


    let period_len = ((adj_end_trial - adj_end_trial) as f64)/n_waves_len;
    let ampli_len = (max_penalty_len - min_penalty__len)/2.0;
    let mid_len = (max_penalty_len+min_penalty__len)/2.0;

    let value_len = ((trial_no-adj_start_trial) as f64)*2.0*std::f64::consts::PI/period_len;
    let mut penalty_len = mid_len + ampli_len*value_len.sin();




    let n_waves = 3.0;
    let min_bonus = 0.0/params::TEST_DATA_SET_SIZE as f64;
    let max_bonus = 1.5/params::TEST_DATA_SET_SIZE as f64;


    let period = ((adj_end_trial - adj_end_trial) as f64)/n_waves;
    let ampli = (max_bonus - min_bonus)/2.0;
    let mid = (max_bonus+min_bonus)/2.0;

    let value = ((trial_no-adj_start_trial) as f64)*2.0*std::f64::consts::PI/period;
    let mut bonus = mid + ampli*value.sin();



    prog.test_fit.unwrap() + (bonus - penalty_len) as f32
}


pub struct MapStats {
    pub best: f32,
    pub worst: f32,
    pub ave: f64,
    pub sd: f64,
    pub count: f32,
}

impl MapStats{
    pub fn to_string(&self)->String{
        format!("best: {}\nworst: {}\nave: {}\ncount: {}", self.best, self.worst, self.ave, self.count)
    }

    pub fn write_update(&self, f: &mut File, n_evals: u64){
        f.write(b"*after ");
        f.write(n_evals.to_string().as_bytes());
        f.write(b" evaluations\n");
        f.write(self.to_string().as_bytes());
        f.write(b"\n\n");
        f.flush();
    }
}

pub enum PutResult{
    Failed,
    Equal,
    Improvement
}
