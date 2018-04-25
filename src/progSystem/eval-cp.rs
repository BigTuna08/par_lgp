use dataMgmt::dataset::{DataSet, TestDataSet, ValidationSet};
use dataMgmt::message::Message;
use params;

pub type EvalFnTest = Fn(Message, &TestDataSet) -> Message;


pub fn eval_program_test(mut request: Message, data: &TestDataSet) -> Message {
    let n_comp_regs = request.genome.n_calc_regs;
    let n_feats = request.genome.features.len();
    let correct = eval_program_corrects(&request.genome, data);

    request.fitness = Some((correct/data.classes.len() as f32));
    request.map_location = Some((n_feats as usize, n_comp_regs as usize));
    request
}

pub trait Evaluator{
    fn eval_program(mut request: Message, data: &TestDataSet) -> Message;
    const DESCRIPTION: &'static str;
}

pub struct CompFeatEvaluator{}
impl Evaluator for CompFeatEvaluator{
    fn eval_program(mut request: Message, data: &TestDataSet) -> Message {
        let n_comp_regs = request.genome.n_calc_regs;
        let n_feats = request.genome.features.len();
        let correct = eval_program_corrects(&request.genome, data);

        request.fitness = Some((correct/data.classes.len() as f32));
        request.map_location = Some((n_feats as usize, n_comp_regs as usize));
        request
    }
    const DESCRIPTION: &'static str = "Uses (n_feat, n_comp_regs)";
}





//penalize progs with higher eff len
pub fn eval_program_test_pen_eff_len(mut request: Message, data: &TestDataSet) -> Message {
    let n_comp_regs = request.genome.n_calc_regs;
    let n_feats = request.genome.features.len();

    let mut correct = eval_program_corrects(&request.genome, data);
    let penalty = (request.genome.get_effective_len(0) as f32)*0.75;
    correct -= penalty;

    request.fitness = Some((correct/data.classes.len() as f32));
    request.map_location = Some((n_feats as usize, n_comp_regs as usize));
    request
}


//fitness penalty for unused comp regs
pub fn eval_program_test_unused_comp_pen(mut request: Message, data: &TestDataSet) -> Message {
    let n_comp_regs = request.genome.n_calc_regs;
    let n_feats = request.genome.features.len();

    let effective_comp_regs = request.genome.get_n_effective_comp_regs(0);
    let penalty = (n_comp_regs as f32 - effective_comp_regs as f32)*0.75;

    let mut correct = eval_program_corrects(&request.genome, data);
    correct -= penalty; //penalize for unused

    request.fitness = Some((correct/data.classes.len() as f32));
    request.map_location = Some((n_feats as usize, n_comp_regs as usize));
    request
}


// use effective comp regs to determine map loc
pub fn eval_program_test_place_eff_comp(mut request: Message, data: &TestDataSet) -> Message {
    let effective_comp_regs = request.genome.get_n_effective_comp_regs(0);
    let n_feats = request.genome.features.len();
    let correct = eval_program_corrects(&request.genome, data);

    request.fitness = Some((correct/data.classes.len() as f32));
    request.map_location = Some((n_feats as usize, effective_comp_regs as usize));
    request
}


// uses eff len
pub fn eval_program_test_with_eff_len(mut request: Message, data: &TestDataSet) -> Message {
    let eff_len = request.genome.get_effective_len(0);
    let n_feats = request.genome.features.len();
    let correct = eval_program_corrects(&request.genome, data);

    request.fitness = Some((correct/data.classes.len() as f32));
    request.map_location = Some((n_feats as usize, eff_len as usize));
    request
}


// uses abs len
pub fn eval_program_test_with_abs_len(mut request: Message, data: &TestDataSet) -> Message {
    let abs_len = request.genome.instructions.len();
    let n_feats = request.genome.features.len();
    let correct = eval_program_corrects(&request.genome, data);

    request.fitness = Some((correct/data.classes.len() as f32));
    request.map_location = Some((n_feats as usize, abs_len as usize));
    request
}


pub fn eval_program_corrects(genome: &super::prog::Program, data: &DataSet) -> f32 {

    let n_comp_regs = genome.n_calc_regs;
    let n_feats = genome.features.len();
    let instr_start = 2 + n_feats;

    let mut correct = 0.0f32;

    for (sample_i, features) in data.feature_iter().enumerate() {

        let mut regs = [0.0f32; params::MAX_REGS];

        let mut reg_val = 0.1;
        for reg in regs.iter_mut() { //semi random initilize regs
            *reg = reg_val;
            reg_val = -(reg_val + 0.05);
        }

        for (i, feature) in genome.features.iter().enumerate() { //load features
            regs[params::MAX_REGS - 1 - i] = features[*feature as usize]
        }

        let prog_output = genome.execute_instructions(regs);

        let prog_result = prog_output >= 0.0;
        if prog_result == data.is_case(sample_i) {correct += 1.0;}
    }
    correct as f32
}


pub fn eval_program_cv(genome: &super::prog::Program, data: &ValidationSet) -> f32 {
    let correct = eval_program_corrects(genome, data);
    correct/ data.classes.len() as f32
}