use dataMgmt::{DataSet, ValidationSet};
use params;
use super::super::{Program, ExecutionRegArray, InstructionResult};
use super::ops;

use std::fs::File;
use std::io::Write;
use std::{thread, time};


pub fn eval_program_corrects(genome: &Program, data: &DataSet) -> f32 {

    let mut correct = 0.0f32;
    let compressed_prog = genome.create_compressed();
    let mut initial_regs = [0.0f32; params::params::MAX_REGS];


    let mut negative = false;
    let mut switch_sign_next = true;

    for i in 1..genome.n_calc_regs as usize{
        if i >= params::params::MAX_REGS{
            break; // out of range, prog shouldnt havve this many regs
        }

        let val = match negative {
            true => -((i+1) as f32),
            false => (i+1) as f32,
        };

//        if i % 2 == 0 { initial_regs[i] = 1.0/val; }
//        else {initial_regs[i] = val; }
        initial_regs[i] = 1.0/val;

        if switch_sign_next{ negative = !negative }

        switch_sign_next = !switch_sign_next //switch every other
    }
//
//    if genome.n_calc_regs > 5{
//        println!("regs {:?}", &initial_regs[0..20]);
//        panic!("done");
//    }



//    let mut reg_val = 0.1;
//    for reg in initial_regs.iter_mut() { //semi random initilize regs
//        *reg = reg_val;
//        reg_val = -(reg_val + 0.05);
//    }

    for record in data.record_iter(){
        let mut regs = initial_regs.clone();

        for (i, feature) in genome.features.iter().enumerate() { //load features
            regs[params::params::MAX_REGS - 1 - i] = record.features[*feature as usize]

        }

        let prog_output = run_prog(&compressed_prog, &mut regs);
        let indetermine_score = 0.5;

        if prog_output.abs() < params::params::EPS { // count zero as no prediction
            correct += indetermine_score;
        }
            else if prog_output.is_nan() {
                // garbage response, treat as wrong
            }
                else {  // good prediction
                    let classification_result = prog_output > 0.0;
                    if classification_result == record.class {correct += 1.0;}
                }


    }
    correct as f32
}

pub fn eval_program_corrects_pos_neg(genome: &Program, data: &DataSet) -> (f32, u8, u8) {

    let mut correct = 0.0f32;
    let compressed_prog = genome.create_compressed();



//
//    if genome.n_calc_regs > 5{
//        println!("regs {:?}", &initial_regs[0..20]);
//        panic!("done");
//    }



//    let mut reg_val = 0.1;
//    for reg in initial_regs.iter_mut() { //semi random initilize regs
//        *reg = reg_val;
//        reg_val = -(reg_val + 0.05);
//    }
    let mut pos_missed = 0;
    let mut neg_missed = 0;

    for record in data.record_iter(){
        let mut regs = super::registers::PROG_REG.clone();

        for (i, feature) in genome.features.iter().enumerate() { //load features
            regs[params::params::MAX_REGS - 1 - i] = record.features[*feature as usize]

        }

        let prog_output = run_prog(&compressed_prog, &mut regs);
        let indetermine_score = 0.5;

        if prog_output.abs() < params::params::EPS { // count zero as no prediction
            correct += indetermine_score;
            pos_missed += 1;
            neg_missed += 1;
        }
        else if prog_output.is_nan() {
            // garbage response, treat as wrong
            pos_missed += 1;
            neg_missed += 1;
        }
        else {  // good prediction
            let classification_result = prog_output > 0.0;
            if classification_result == record.class {
                if classification_result { // positive case
                    correct += 1.0;
                }
                else {
                    correct += 1.0;
                }
            }
            else {
                if classification_result { // positive case
                    pos_missed += 1;
                }
                else {
                    neg_missed += 1;
                }
            }
        }


    }

    (correct as f32, pos_missed, neg_missed)
}




pub fn run_prog(prog: &Program, regs: &mut ExecutionRegArray) -> f32 {

    let mut skip_count = 0u8; // used to implement branches

    for instr in prog.instructions.iter() {
        if skip_count > 0 {
            skip_count -= 1;
            continue;
        }

        let result = ops::execute_op(instr, regs);
        match result {
            InstructionResult::Value(result) => regs[instr.dest as usize] = result,
            InstructionResult::Skip(result) => skip_count = result,
            InstructionResult::Terminate => break,
            InstructionResult::NoOp => (),
        }
    }
    regs[0]
}



pub fn eval_program_cv(genome: &Program, data: &ValidationSet) -> f32 {
    let correct = eval_program_corrects(genome, data);
    correct/ data.size() as f32
}


pub fn eval_program_corrects_testing_with_assert(genome: &Program, data: &DataSet) -> f32 {

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
        let mut regs2 = initial_regs.clone();

        for (i, feature) in genome.features.iter().enumerate() { //load features
            regs[params::params::MAX_REGS - 1 - i] = record.features[*feature as usize];
            regs2[params::params::MAX_REGS - 1 - i] = record.features[*feature as usize];
        }

        let c_prog_output = run_prog(&compressed_prog, &mut regs);
        let prog_output = run_prog(&genome, &mut regs2);
//        println!("reg={}\tcomp={}", prog_output, c_prog_output);
//        log_after_error(genome, &format!("err-{}-{}.txt", prog_output, c_prog_output));
        if (prog_output - c_prog_output).abs() > params::params::EPS{
            println!("logging bad!");
            log_after_error(genome, &format!("err-{}-{}.txt", prog_output, c_prog_output));

//            let ten_millis = time::Duration::from_millis(10);
//            thread::sleep(ten_millis);
            panic!("bad excision by {}", (prog_output-c_prog_output).abs());
        }
//        assert_eq!(prog_output, c_prog_output);

//        let classification_result = c_prog_output >= 0.0;
//        if classification_result == record.class {correct += 1.0;}
    }
    correct as f32
}


pub fn log_after_error(genome: &Program, file_name: &str){
    let mut f = File::create(file_name).unwrap();
    genome.write_header(&mut f);
    f.write(b"\n");
    genome.write_self_words(&mut f);
    f.write(b"\n");
    genome.write_effective_self_words(&mut f);
    f.write(b"\n");
    f.write(b"\n");
}