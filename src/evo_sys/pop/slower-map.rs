use config::Config;
use dataMgmt::dataset::ValidationSet;
use dataMgmt::logger::GenoEval;
use dataMgmt::message::EvalResult;
use evo_sys;
use evo_sys::prog::prog::Program;
use indexmap::IndexMap;
use params;
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
            Some(prog) => prog.cv_fit.unwrap(),
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


    //pick random item from geno map and return random mutated copy
    pub fn get_simple_mutated_genome_rand(&self) -> Program {
        self.prog_map.get_index(rand::thread_rng().gen_range(0, self.prog_map.len())).unwrap().1.test_mutate_copy()
    }


    pub fn write_pop_info(&self, file_name: &str, eval: PopEval){
        let mut f = File::create(file_name).unwrap();

        for row_i in 0.. params::MAP_ROWS{
            for col_i in 0.. params::MAP_COLS{

                let value =
                    if self.has_prog(&(row_i, col_i)){
                        match eval {
                            PopEval::TestFit => self.get_test_fit(&(row_i, col_i)),
                            PopEval::CV => self.get_cv_fit(&(row_i, col_i)),
                            PopEval::Geno(eval) => eval(self.prog_map.get(&(row_i, col_i)).unwrap()),
                        }

                    }
                    else {
                       params::MIN_FIT
                    };

                f.write(value.to_string().as_bytes());
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

}


impl ResultMap{

    pub fn has_prog(&self, inds: &(usize, usize))->bool{
        match self.prog_map.get(inds) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn get_pop_stats(&self, eval: PopEval) -> PopStats {
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
        PopStats {best, worst, ave, sd:vari.sqrt(), count: count as f32}
    }


    pub fn update_cv(&mut self, data: &ValidationSet) {
        for row_i in 0.. params::MAP_ROWS{
            for col_i in 0.. params::MAP_COLS{
                if let Some(genome) = self.prog_map.get_mut(&(row_i, col_i)) {
                    match genome.cv_fit {
                        Some(_) => (),
                        None =>  genome.cv_fit = Some(evo_sys::prog::eval::eval_program_cv(genome, &data)),
                    }
                }
            }
        }
    }

}


//fn get_adjusted_fit(prog: &Program, trial_no: u64)->f32{
//    let period = 2_000_000.0;
//
//    let min = -1.5/params::TEST_DATA_SET_SIZE as f64;
//    let max = 1.5/params::TEST_DATA_SET_SIZE as f64;
//
//    let ampli = (max - min)/2.0;
//    let mid = (max+min)/2.0;
//
//    let value = (trial_no as f64)*2.0*std::f64::consts::PI/period;
//    let mut penalty = mid + ampli*value.sin();
//    penalty *= prog.get_effective_len(0) as f64;
//    prog.test_fit.unwrap() - penalty as f32
//}
//
//





