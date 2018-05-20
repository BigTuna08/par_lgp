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

        let prog_output = run_prog(&compressed_prog, &mut regs);

        let classification_result = prog_output >= 0.0;
        if classification_result == record.class {correct += 1.0;}
    }
    correct as f32
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