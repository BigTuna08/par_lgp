use dataMgmt::dataset::ValidationSet;
use dataMgmt::logger::GenoEval;
use dataMgmt::message::Message;
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


pub struct ResultMap{
    fit_map: IndexMap<(usize, usize),f32>,
    cv_map: IndexMap<(usize, usize),f32>,
    geno_map: IndexMap<(usize, usize),Program>,
}


impl ResultMap {
    pub fn new() -> ResultMap {
        ResultMap {
            fit_map: IndexMap::new(),
            cv_map: IndexMap::new(),
            geno_map: IndexMap::new(),
        }
    }

    pub fn get_test_fit(&self, inds: &(usize, usize)) -> f32 {
        match self.fit_map.get(inds) {
            Some(fit) => *fit,
            None => params::MIN_FIT,
        }
    }

    pub fn get_cv_fit(&self, inds: &(usize, usize))-> f32 {
        match self.cv_map.get(inds) {
            Some(fit) => *fit,
            None => params::MIN_FIT,
        }
    }

    pub fn put(&mut self, val:Message, inds: &(usize, usize)) {
        self.fit_map.insert(*inds, val.fitness.unwrap());
        self.geno_map.insert(*inds, val.genome);
        self.cv_map.insert(*inds, params::MIN_FIT);
    }


    //returns true only if made an improvement
    pub fn try_put(&mut self, new_entry: Message) -> PutResult {
        let inds = &new_entry.map_location.unwrap();
        let new_fit = new_entry.fitness.unwrap();
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
    pub fn try_put_trial_based(&mut self, new_entry: Message) -> PutResult {
        let inds = &new_entry.map_location.unwrap();
        let new_fit = new_entry.fitness.unwrap();
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
        self.geno_map.get_index(rand::thread_rng().gen_range(0, self.geno_map.len())).unwrap().1.test_mutate_copy()
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


    pub fn write_genos(&self, file_name: &str, feat_names: &Vec<String>){
        let mut f = File::create(file_name).unwrap();
        for row_i in 0..params::MAP_ROWS {
            for col_i in 0..params::MAP_COLS {
                let geno = self.geno_map.get(&(row_i, col_i));
                if let Some(ref genome) = geno{
                    f.write(b"(");
                    f.write(row_i.to_string().as_bytes());
                    f.write(b",");
                    f.write(col_i.to_string().as_bytes());
                    f.write(b")");
                    f.write(b"\n");
                    genome.write_effective_self_words(&mut f, feat_names);
//                    genome.write_self_words(&mut f, feat_names);
                }
            }
        }
    }


    pub fn write_genos_abs_len(&self, file_name: &str){
        let mut f = File::create(file_name).unwrap();
        for row_i in 0..params::MAP_ROWS {
            for col_i in 0..params::MAP_COLS {

                if let Some(ref genome) = self.geno_map.get(&(row_i, col_i)){
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

                if let Some(ref genome) = self.geno_map.get(&(row_i, col_i)){
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
        let i = rand::thread_rng().gen_range(0, self.fit_map.len());
        self.fit_map.get_index(i).unwrap().0
    }

}


impl ResultMap{
    pub fn get_test_stats(&self) -> MapStats{
        let mut best = std::f32::MIN;
        let mut worst = std::f32::MAX;
        let mut ave = 0.0f64;
        let mut count = 0.0;

        for row_i in 0.. params::MAP_ROWS{
            for col_i in 0.. params::MAP_COLS{
                let fit = self.get_test_fit(&(row_i, col_i));

                if fit > params::MIN_FIT{
                    ave += fit as f64;
                    count += 1.0;
                    if fit > best {best=fit;}
                    if fit < worst {worst=fit;}
                }

            }
        }
        ave = ave/count;

        let mut vari = 0.0;
        for row_i in 0.. params::MAP_ROWS{
            for col_i in 0.. params::MAP_COLS{
                let fit = self.get_test_fit(&(row_i, col_i));
                if fit != params::MIN_FIT{
                    vari += (fit as f64-ave).powi(2);
                }
            }
        }
        vari /= count;

        MapStats {best, worst, ave, sd:vari.sqrt(), count: count as f32}
    }


    pub fn update_cross_validation(&mut self, data: &ValidationSet) -> MapStats {
        let mut best = std::f32::MIN;
        let mut worst = std::f32::MAX;
        let mut ave = 0.0f64;
        let mut count = 0.0;




        for row_i in 0.. params::MAP_ROWS{
            for col_i in 0.. params::MAP_COLS{

                if self.get_cv_fit(&(row_i, col_i)) == params::MIN_FIT{ //new since last, or doesnt exist

                    if let Some(ref genome) = self.geno_map.get(&(row_i, col_i)) { //does exist
                        let fit = progSystem::eval::eval_program_cv(genome, &data);
                        self.cv_map.insert((row_i, col_i),fit);
                        ave += fit as f64;
                        count += 1.0;
                        if fit > best {best=fit;}
                        if fit < worst {worst=fit;}
                    }
                }else{

                }

                match self.geno_map.get(&(row_i, col_i)) {
                    Some(ref genome) => {
                        let cv_map_fit = self.get_cv_fit(&(row_i, col_i));

                        let fit = if cv_map_fit > params::MIN_FIT {cv_map_fit}  //can use old
                                        else {
                                            let new_fit = progSystem::eval::eval_program_cv(genome, &data);
                                            self.cv_map.insert((row_i, col_i), new_fit);
                                            new_fit
                                        };

                        ave += fit as f64;
                        count += 1.0;
                        if fit > best {best=fit;}
                        if fit < worst {worst=fit;}

                    },
                    None => ()
                }
            }
        }
        ave = ave/count;

        let mut vari = 0.0;
        for row_i in 0.. params::MAP_ROWS{
            for col_i in 0.. params::MAP_COLS{
                let fit = self.get_cv_fit(&(row_i, col_i));
                if fit != params::MIN_FIT{
                    vari += (fit as f64-ave).powi(2);
                }
            }
        }
        vari /= count;

        MapStats {best, worst, ave, sd:vari.sqrt(), count:count as f32}
    }


    pub fn get_geno_stats(&mut self, eval: &GenoEval) -> MapStats {
        let mut best = std::f32::MIN;
        let mut worst = std::f32::MAX;
        let mut ave = 0.0f64;
        let mut count = 0.0;

        for row_i in 0.. params::MAP_ROWS{
            for col_i in 0.. params::MAP_COLS{
                if let Some(ref genome) = self.geno_map.get(&(row_i, col_i)) { //does exist
                    let score = eval(*genome);
                    ave += score as f64;
                    count += 1.0;
                    if score > best {best=score;}
                    if score < worst {worst=score;}
                }

            }
        }
        ave = ave/count;

        let mut vari = 0.0;
        for row_i in 0.. params::MAP_ROWS{
            for col_i in 0.. params::MAP_COLS{
                let fit = self.get_cv_fit(&(row_i, col_i));
                if fit != params::MIN_FIT{
                    vari += (fit as f64-ave).powi(2);
                }
            }
        }
        vari /= count;

        MapStats {best, worst, ave, sd:vari.sqrt(), count:count as f32}
    }
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