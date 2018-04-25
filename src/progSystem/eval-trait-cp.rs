use dataMgmt::dataset::{DataSet, TestDataSet, ValidationSet};
use dataMgmt::message::Message;
use params;

pub type EvalFnTest = Fn(Message, &TestDataSet) -> Message;

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


pub struct CompFeatEvaluatorPenEL{}
impl Evaluator for CompFeatEvaluatorPenEL{
    fn eval_program(mut request: Message, data: &TestDataSet) -> Message {
        let n_comp_regs = request.genome.n_calc_regs;
        let n_feats = request.genome.features.len();

        let mut correct = eval_program_corrects(&request.genome, data);
        let penalty = (request.genome.get_effective_len(0) as f32)*0.75;
        correct -= penalty;

        request.fitness = Some((correct/data.classes.len() as f32));
        request.map_location = Some((n_feats as usize, n_comp_regs as usize));
        request
    }
    const DESCRIPTION: &'static str = "Uses (n_feat, n_comp_regs), penalizes effective prog len";
}


pub struct CompFeatEvaluatorPenUnusedComp{}
impl Evaluator for CompFeatEvaluatorPenUnusedComp{
    fn eval_program(mut request: Message, data: &TestDataSet) -> Message {
        let n_comp_regs = request.genome.n_calc_regs;
        let n_feats = request.genome.features.len();

        let mut correct = eval_program_corrects(&request.genome, data);
        let penalty = (request.genome.get_effective_len(0) as f32)*0.75;
        correct -= penalty;

        request.fitness = Some((correct/data.classes.len() as f32));
        request.map_location = Some((n_feats as usize, n_comp_regs as usize));
        request
    }
    const DESCRIPTION: &'static str = "Uses (n_feat, n_comp_regs), penalizes unused comp regs";
}


pub struct EffCompFeatEvaluator{}
impl Evaluator for EffCompFeatEvaluator{
    fn eval_program(mut request: Message, data: &TestDataSet) -> Message {
        let effective_comp_regs = request.genome.get_n_effective_comp_regs(0);
        let n_feats = request.genome.features.len();
        let correct = eval_program_corrects(&request.genome, data);

        request.fitness = Some((correct/data.classes.len() as f32));
        request.map_location = Some((n_feats as usize, effective_comp_regs as usize));
        request
    }
    const DESCRIPTION: &'static str = "Uses (n_feat, effctive n_comp_regs)";
}


pub struct EffLenFeatEvaluator{}
impl Evaluator for EffLenFeatEvaluator{
    fn eval_program(mut request: Message, data: &TestDataSet) -> Message {
        let eff_len = request.genome.get_effective_len(0);
        let n_feats = request.genome.features.len();
        let correct = eval_program_corrects(&request.genome, data);

        request.fitness = Some((correct/data.classes.len() as f32));
        request.map_location = Some((n_feats as usize, eff_len as usize));
        request
    }
    const DESCRIPTION: &'static str = "Uses (n_feat, effctive len)";
}


pub struct AbsLenFeatEvaluator{}
impl Evaluator for AbsLenFeatEvaluator{
    fn eval_program(mut request: Message, data: &TestDataSet) -> Message {
        let abs_len = request.genome.instructions.len();
        let n_feats = request.genome.features.len();
        let correct = eval_program_corrects(&request.genome, data);

        request.fitness = Some((correct/data.classes.len() as f32));
        request.map_location = Some((n_feats as usize, abs_len as usize));
        request
    }
    const DESCRIPTION: &'static str = "Uses (n_feat, abs len)";
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