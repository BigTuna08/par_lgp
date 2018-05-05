use dataMgmt::{DataSet, ValidationSet};
use params;
use super::super::Program;


pub fn eval_program_corrects(genome: &Program, data: &DataSet) -> f32 {

    let mut correct = 0.0f32;
    let compressed_prog = genome.create_compressed();
    let mut initial_regs = [0.0f32; params::params::MAX_REGS];


    let mut reg_val = 0.1;
    for reg in initial_regs.iter_mut() { //semi random initilize regs
        *reg = reg_val;
        reg_val = -(reg_val + 0.05);
    }

    for record in data.record_iter(){
        let mut regs = initial_regs.clone();

        for (i, feature) in genome.features.iter().enumerate() { //load features
            regs[params::params::MAX_REGS - 1 - i] = record.features[*feature as usize]
        }

        let prog_output = compressed_prog.execute_instructions(regs);

        let classification_result = prog_output >= 0.0;
        if classification_result == record.class {correct += 1.0;}
    }
    correct as f32
}



pub fn eval_program_cv(genome: &Program, data: &ValidationSet) -> f32 {
    let correct = eval_program_corrects(genome, data);
    correct/ data.size() as f32
}
