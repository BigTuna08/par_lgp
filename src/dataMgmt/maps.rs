use progSystem;
use progSystem::prog::Program;
use params;
use dataMgmt::message::Message;
use dataMgmt::dataset::ValidationSet;

use rand;
use rand::Rng;
use std;
use std::fs::File;
use std::io::Write;



pub struct ResultMap{
    fit_map: [[f32; params::MAP_COLS]; params::MAP_ROWS],
    cv_map: [[f32; params::MAP_COLS]; params::MAP_ROWS],
    geno_map: [[Option<Program>; params::MAP_COLS]; params::MAP_ROWS],
}


impl ResultMap {
    pub fn new() -> ResultMap {
        ResultMap {
            fit_map: [[params::MIN_FIT; params::MAP_COLS]; params::MAP_COLS],
            cv_map: [[params::MIN_FIT; params::MAP_COLS]; params::MAP_COLS],
            geno_map:
            [[None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None], ]
        }
    }

    pub fn get_fit(&self, inds: &(usize, usize))-> f32 {
        self.fit_map[inds.0][inds.1]
    }

    pub fn put(&mut self, val:Message) {
        let inds = &val.map_location.unwrap();
        self.fit_map[inds.0][inds.1] = val.fitness.unwrap();
        self.geno_map[inds.0][inds.1] = Some(val.genome);
        self.cv_map[inds.0][inds.1] == params::MIN_FIT; //dont do cv until needed
    }


    //returns true only if made an improvement
    pub fn try_put(&mut self, new_entry: Message) -> PutResult {
        let inds = &new_entry.map_location.unwrap();
        let new_fit = &new_entry.fitness.unwrap();


        if inds.0 >= params::MAP_ROWS || inds.1 >= params::MAP_COLS || new_fit < &self.fit_map[inds.0][inds.1] {
            return PutResult::Failed
        }

        else if new_fit > &self.fit_map[inds.0][inds.1] {
            self.put(new_entry);
            return PutResult::Improvement
        }

        else if rand::thread_rng().gen_weighted_bool(params::REPLACE_EQ_FIT) {
            self.put(new_entry);
            return PutResult::Equal
        }
        else { PutResult::Failed }//equal fit but failed to replace
    }

    //now only chooses from one spot
    pub fn get_simple_mutated_genome_rand(&self) -> Program {

        let inds = self.get_sample_inds();
//        println!("got them sure");
        match self.geno_map[inds.0][inds.1] {
            Some(ref genome) => {
//                println!("got geno from {:?}", inds);
                genome.test_mutate_copy()
            },
            None => {
                println!("None");
                Program::new_default()
            }
        }
    }



    pub fn write_test_fits(&self, file_name: &str){
        let mut f = File::create(file_name).unwrap();
        for row in self.fit_map.iter(){

            let mut print_str: String = row.iter().fold(String::new(),
                                                        |mut acc, &x| {acc.push_str(&x.to_string()); acc.push_str("\t"); acc} );

            print_str.push_str("\n");
            f.write(print_str.as_bytes());
        }
    }


    pub fn write_cv_fits(&self, file_name: &str){
        let mut f = File::create(file_name).unwrap();
        for row in self.cv_map.iter(){

            let mut print_str: String = row.iter().fold(String::new(),
                                                        |mut acc, &x| {acc.push_str(&x.to_string()); acc.push_str("\t"); acc} );

            print_str.push_str("\n");
            f.write(print_str.as_bytes());
        }
    }




    pub fn write_genos(&self, file_name: &str, feat_names: &Vec<String>){
        let mut f = File::create(file_name).unwrap();
        for (row_i, row) in self.geno_map.iter().enumerate(){
            for (col_i, geno) in row.iter().enumerate(){
                if let Some(ref genome) = *geno{
                    f.write(b"(");
                    f.write(row_i.to_string().as_bytes());
                    f.write(b",");
                    f.write(col_i.to_string().as_bytes());
                    f.write(b")");
                    f.write(b"\n");
                    genome.write_self(&mut f, feat_names);
                }
            }
        }
    }


    pub fn write_genos_abs_len(&self, file_name: &str){
        let mut f = File::create(file_name).unwrap();
        for (row_i, row) in self.geno_map.iter().enumerate(){
            for (col_i, geno) in row.iter().enumerate(){
                if let Some(ref genome) = *geno{
                    f.write(genome.instructions.len().to_string().as_bytes());
                    f.write(b"\t");
                }else {
                    f.write(b"-1.0\t");
                }
            }
            f.write(b"\n");
        }
    }

    pub fn write_genos_eff_len(&self, file_name: &str){
        let mut f = File::create(file_name).unwrap();
        for (row_i, row) in self.geno_map.iter().enumerate(){
            for (col_i, geno) in row.iter().enumerate(){
                if let Some(ref genome) = *geno{
                    f.write(genome.get_effective_len(0).to_string().as_bytes());
                    f.write(b"\t");
                }else {
                    f.write(b"-1.0\t");
                }
            }
            f.write(b"\n");
        }
    }


    fn get_sample_inds(&self) -> (usize, usize) {
//        println!("Getting inds");
        let mut rng = rand::thread_rng();
        let mut inds = ( rng.gen_range(0, params::MAP_ROWS) , rng.gen_range(0, params::MAP_COLS));
        while self.fit_map[inds.0][inds.1] == params::MIN_FIT{
            inds = ( rng.gen_range(0, params::MAP_ROWS) , rng.gen_range(0, params::MAP_COLS));
        }
//        println!("done");
        inds
    }


    pub fn get_stats(&self) -> MapStats{
        let mut best = std::f32::MIN;
        let mut worst = std::f32::MAX;
        let mut ave = 0.0;
        let mut count = 0.0;

        for row_i in 0.. params::MAP_ROWS{
            for col_i in 0.. params::MAP_COLS{
                let fit = self.fit_map[row_i][col_i];

                if fit > params::MIN_FIT{
                    ave += fit;
                    count += 1.0;
                    if fit > best {best=fit;}
                    if fit < worst {worst=fit;}
                }

            }
        }
        ave = ave/count;
        MapStats {best, worst, ave, count}
    }


    pub fn update_cross_validation(&mut self, data: ValidationSet) -> MapStats {
        let mut best = std::f32::MIN;
        let mut worst = std::f32::MAX;
        let mut ave = 0.0;
        let mut count = 0.0;

        for row_i in 0.. params::MAP_ROWS{
            for col_i in 0.. params::MAP_COLS{
                if self.cv_map[row_i][col_i] == params::MIN_FIT{ //new since last, or doesnt exist
                    if let Some(ref genome) = self.geno_map[row_i][col_i] { //does exist
                        let fit = progSystem::eval::eval_program_cv(genome, &data);
                        self.cv_map[row_i][col_i] = fit;
                        ave += fit;
                        count += 1.0;
                        if fit > best {best=fit;}
                        if fit < worst {worst=fit;}
                    }
                }
            }
        }
        ave = ave/count;
        MapStats {best, worst, ave, count}
    }


}

pub struct MapStats {
    best: f32,
    worst: f32,
    ave: f32,
    count: f32,
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