use params;

use super::super::{ProgramOperation, Instruction, InstructionResult, ExecutionRegArray, InstructionType};


pub fn execute_op(instr: &Instruction, regs: &ExecutionRegArray) -> InstructionResult{
    match instr.op {
        0 => add(instr, regs),
        1 => subt(instr, regs),
        2 => mult(instr, regs),
        3 => pdiv(instr, regs),
        4 => pow(instr, regs),
        5 => log(instr, regs),
        6 => big(instr, regs),
        7 => kill(instr, regs),
        _ => panic!("invalid op # {} (during execution) ", instr.op),
    }
}


pub fn get_type(instr: &Instruction) -> InstructionType{
    match instr.op {
        0 => InstructionType::Value,
        1 => InstructionType::Value,
        2 => InstructionType::Value,
        3 => InstructionType::Value,
        4 => InstructionType::Value,
        5 => InstructionType::Value,
        6 => {
            if instr.src1 == instr.src2 { // x > x is never true
                InstructionType::NoOp
            }
            else {
                InstructionType::Skip
            }
        }
        7 => InstructionType::Terminate,
        _ => panic!("invalid op # {} (getting type)", instr.op),
    }
}


pub fn get_name(instr: &Instruction) -> &str{
    match instr.op {
        0 => "add",
        1 => "subt",
        2 => "mult",
        3 => "pdiv",
        4 => "pow",
        5 => "log",
        6 => "big",
        7 => "kill",
        _ => panic!("invalid op # {} (getting name)", instr.op),
    }
}

pub fn formatted_string(instr: &Instruction, src1: &str, src2: &str) -> String{
    match get_type(instr) {
        InstructionType::Value => format!("${}\t=\t{}\t{}\t{}", instr.dest, get_name(instr), src1, src2),

        InstructionType::Skip => match instr.op {
            6 => format!("Skip next if {}\t>\t{}", src1, src2),
            _ => panic!("invalid op code for skip instr! {} (formatted string)")
        }

        InstructionType::Terminate => match instr.op {
            7 => format!("QUIT"),
            _ => panic!("invalid op code for terminate instr! {} (formatted string)")
        }

        InstructionType::NoOp => format!("NO-OP"),
    }
}

fn add(instr: &Instruction, regs: &ExecutionRegArray) -> InstructionResult {
    InstructionResult::Value(regs[instr.src1 as usize] + regs[instr.src2 as usize])
}

fn subt(instr: &Instruction, regs: &ExecutionRegArray) -> InstructionResult {
    InstructionResult::Value(regs[instr.src1 as usize] - regs[instr.src2 as usize])
}

fn mult(instr: &Instruction, regs: &ExecutionRegArray) -> InstructionResult {
    InstructionResult::Value(regs[instr.src1 as usize] * regs[instr.src2 as usize])
}

fn pdiv(instr: &Instruction, regs: &ExecutionRegArray) -> InstructionResult {
    if regs[instr.src2 as usize].abs() > params::params::EPS
        {InstructionResult::Value(regs[instr.src1 as usize]/regs[instr.src2 as usize])}
    else { InstructionResult::Value(regs[instr.src1 as usize]) } //protect if regs[instr.src2 as usize]) ~= 0
}

fn pow(instr: &Instruction, regs: &ExecutionRegArray) -> InstructionResult {
    InstructionResult::Value(regs[instr.src1 as usize].powf(regs[instr.src2 as usize]))
}

fn log(instr: &Instruction, regs: &ExecutionRegArray) -> InstructionResult {
    InstructionResult::Value(regs[instr.src1 as usize].log(regs[instr.src2 as usize]))
}

//branch if greater
fn big(instr: &Instruction, regs: &ExecutionRegArray) -> InstructionResult{
    if regs[instr.src1 as usize] > regs[instr.src2 as usize] {InstructionResult::Skip(1)} //branch
    else {InstructionResult::NoOp} //dont
}

//send message to kill program execution
fn kill(instr: &Instruction, regs: &ExecutionRegArray) -> InstructionResult {
    InstructionResult::Terminate
}
