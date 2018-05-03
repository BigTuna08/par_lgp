use dataMgmt::dataset::{DataSet, TestDataSet, ValidationSet};
use dataMgmt::message::Message;
use params;
use rand::Rng;
use rand::thread_rng;


//pub type EvalFnTest = &'static Fn(Message, &TestDataSet) -> Message;

//pub static EVALS: &'static [&Fn(Message, &TestDataSet) -> Message] =
//    &[&ev_f_c,
//    &ev_pen_eff_len,
//    &ev_pen_un_comp,
//    &ev_eff_comp,
//    &ev_eff_len,
//    &ev_abs_len];

pub static EVALS_DESC: &'static [&'static str] =
    &["0 place with (features, comp regs)",
        "1 place with (features, comp regs) - pen eff len",
        "2 place with (features, comp regs) - pen unused comp regs",
        "3 place with (features, effective comp regs)",
        "4 place with (features, effective len)",
        "5 place with (features, abs len)",
        "6 place with (features, y), where y is either abs_len or n_comp reg (split range)",
        "7 place with (n_comp_reg, y), where y is either abs_len or n_feat (split range)",
        "8 place with (n_feats, n_comp_reg, eff_len) mapped to 2d grid",
        "9 place with (abs len, comp_reg), penalize effective feats",
        "10 place with (abs len, comp_reg), penalize effective len",
        "11 place with (effective feats, n_comp regs)",
        "12 big square (n_feat, eff_len) small (n_comp, rand)",
        "13 big square (n_feat, abs_len) small (n_comp, rand)",
        "14 big square (n_eff_feat, eff_len) small (n_eff_comp, rand), squares all resolution 1",
        "15 like 13 but rands get noise",
        "16 like 13 but rands get diff penalites",
        "17 place with (comp res, abs len)",
        "18 place with (eff comp regs, eff len)",
//        "17 like 0 but vary pens (-1,1) long period ",
//        "18 like 0 but vary pens (-2,2) long period ",
//        "19 like 0 but vary pens (-1,1) short period ",
//        "20 like 0 but vary pens (-2,2) short period ",
//        "21 like 0 but vary pens (-1,1) very long period ",
//        "22 like 0 but vary pens (0,2) long period ",
//        "23 like 0 but vary pens (0,3) very long period ",
//        "24 like 0 but vary pens (-2,4) very long period ",
//        "25 varpen meant to run with 25M evals",
    ];

//
//pub fn get_fn(n: usize) -> &'static Fn(Message, &TestDataSet) -> Message {
//    match n {
////        0 => &ev_f_c,
////        1 => &ev_pen_eff_len,
////        2 => &ev_pen_un_comp,
////        3 => &ev_eff_comp,
////        4 =>&ev_eff_len,
////        5 => &ev_abs_len,
////        6 => &ev_three,
////        7 => &ev_three2,
////        8 => &ev_three_smart,
////        9 => &ev_abs_comp_pen_eff_feat,
////        10 => &ev_abs_comp_pen_eff_len,
////        11 => &ev_eff_feat_comp,
////        12 => &ev_four,
////        13 => &ev_four_abs_len,
////        14 => &ev_four_effective_small,
////        15 => &ev_four_abs_len_noise,
////        16 => &ev_four_abs_len_pen,
//        17 => &ev_17,
////        18 => &ev_18,
////        19 => &ev_f_c_varpen3,
////        20 => &ev_f_c_varpen4,
////        21 => &ev_f_c_varpen5,
////        22 => &ev_f_c_varpen6,
////        23 => &ev_f_c_varpen7,
////        24 => &ev_f_c_varpen8,
////        25 => &ev_f_c_varpen9,
//        _ => panic!("bad spot!!"),
//    }
//}

//(comp_regs, abs_len)
//fn ev_17(mut request: Message, data: &TestDataSet) -> Message {
//    let row = request.genome.n_calc_regs;
//    let col = request.genome.get_abs_len();
//
//    let correct = eval_program_corrects(&request.genome, data);
//
//    request.genome.test_fit = Some(correct/data.size() as f32);
//    request.map_location = Some((row as usize, col as usize));
//    request
//}

//
////(effective comp_regs, eff_len)
//fn ev_18(mut request: EvalResult, data: &TestDataSet) -> EvalResult {
//    let row = request.genome.get_n_effective_comp_regs(0);
//    let col = request.genome.get_effective_len(0);
//
//    let correct = eval_program_corrects(&request.genome, data);
//
//    request.genome.test_fit = Some(correct/data.size() as f32);
//    request.map_location = Some((row as usize, col as usize));
//    request
//}
//
//
//fn ev_f_c(mut request: EvalResult, data: &TestDataSet) -> EvalResult {
//    let n_comp_regs = request.genome.n_calc_regs;
//    let n_feats = request.genome.features.len();
//    let correct = eval_program_corrects(&request.genome, data);
//
//    request.genome.test_fit = Some(correct/data.size() as f32);
//    request.map_location = Some((n_feats as usize, n_comp_regs as usize));
//    request
//}
//
////penalize progs with higher eff len
//pub fn ev_pen_eff_len(mut request: EvalResult, data: &TestDataSet) -> EvalResult {
//    let n_comp_regs = request.genome.n_calc_regs;
//    let n_feats = request.genome.features.len();
//
//    let mut correct = eval_program_corrects(&request.genome, data);
//    let penalty = (request.genome.get_effective_len(0) as f32)*0.75;
//    correct -= penalty;
//
//    request.genome.test_fit = Some(correct/data.size() as f32);
//    request.map_location = Some((n_feats as usize, n_comp_regs as usize));
//    request
//}
//
//
////fitness penalty for unused comp regs
//pub fn ev_pen_un_comp(mut request: EvalResult, data: &TestDataSet) -> EvalResult {
//    let n_comp_regs = request.genome.n_calc_regs;
//    let n_feats = request.genome.features.len();
//
//    let effective_comp_regs = request.genome.get_n_effective_comp_regs(0);
//    let penalty = (n_comp_regs as f32 - effective_comp_regs as f32)*0.75;
//
//    let mut correct = eval_program_corrects(&request.genome, data);
//    correct -= penalty; //penalize for unused
//
//    request.genome.test_fit = Some(correct/data.size() as f32);
//    request.map_location = Some((n_feats as usize, n_comp_regs as usize));
//    request
//}
//
//
//// use effective comp regs to determine map loc
//pub fn ev_eff_comp(mut request: EvalResult, data: &TestDataSet) -> EvalResult {
//    let effective_comp_regs = request.genome.get_n_effective_comp_regs(0);
//    let n_feats = request.genome.features.len();
//    let correct = eval_program_corrects(&request.genome, data);
//
//    request.genome.test_fit = Some(correct/data.size() as f32);
//    request.map_location = Some((n_feats as usize, effective_comp_regs as usize));
//    request
//}
//
//
//// uses eff len
//pub fn ev_eff_len(mut request: EvalResult, data: &TestDataSet) -> EvalResult {
//    let eff_len = request.genome.get_effective_len(0);
//    let n_feats = request.genome.features.len();
//    let correct = eval_program_corrects(&request.genome, data);
//
//    request.genome.test_fit = Some(correct/data.size() as f32);
//    request.map_location = Some((n_feats as usize, eff_len as usize));
//    request
//}
//
//
//// uses abs len
//pub fn ev_abs_len(mut request: EvalResult, data: &TestDataSet) -> EvalResult {
//    let abs_len = request.genome.instructions.len();
//    let n_feats = request.genome.features.len();
//    let correct = eval_program_corrects(&request.genome, data);
//
//    request.genome.test_fit = Some(correct/data.size() as f32);
//    request.map_location = Some((n_feats as usize, abs_len as usize));
//    request
//}
//
//
//pub fn ev_three(mut request: EvalResult, data: &TestDataSet) -> EvalResult {
//    let abs_len = request.genome.instructions.len();
//    let n_feats = request.genome.features.len();
//    let n_comp_regs = request.genome.n_calc_regs;
//    let correct = eval_program_corrects(&request.genome, data);
//
//    let y = if thread_rng().gen_weighted_bool(2) {
//        abs_len
//    }else {
//        params::params::MAP_ROWS/2 + n_comp_regs as usize
//    };
//
//    request.genome.test_fit = Some(correct/data.size() as f32);
//    request.map_location = Some((n_feats as usize, y));
//    request
//}
//
//
//pub fn ev_three2(mut request: EvalResult, data: &TestDataSet) -> EvalResult {
//    let abs_len = request.genome.instructions.len();
//    let n_feats = request.genome.features.len();
//    let n_comp_regs = request.genome.n_calc_regs;
//    let correct = eval_program_corrects(&request.genome, data);
//
//    let y = if thread_rng().gen_weighted_bool(2) {
//        abs_len
//    }else {
//        params::params::MAP_ROWS/2 + n_feats as usize
//    };
//
//    request.genome.test_fit = Some(correct/data.size() as f32);
//    request.map_location = Some((n_comp_regs as usize, y));
//    request
//}
//
//
//pub fn ev_three_smart(mut request: EvalResult, data: &TestDataSet) -> EvalResult {
//    let n_feats = request.genome.features.len();
//    let n_comp_regs = request.genome.n_calc_regs;
//
//    let feat_ind = get_mapped_index(n_feats);
//    let comp_ind = get_mapped_index(n_comp_regs as usize);
//    let eff_len = request.genome.get_effective_len(0);
//
//    let correct = eval_program_corrects(&request.genome, data);
//
//    request.genome.test_fit = Some(correct/data.size() as f32);
//    request.map_location = Some(reduce3to2dim(feat_ind, comp_ind, eff_len));
//    request
//}
//
////place abs len, comp_reg, penalize effective feats
//pub fn ev_abs_comp_pen_eff_feat(mut request: EvalResult, data: &TestDataSet) -> EvalResult {
//    let n_comp_regs = request.genome.n_calc_regs;
//    let n_feats_eff = request.genome.get_n_effective_feats(0);
//    let abs_len = request.genome.instructions.len();
//
//    let mut correct = eval_program_corrects(&request.genome, data);
//    let penalty = (n_feats_eff as f32)*0.75;
//    correct -= penalty;
//
//    request.genome.test_fit = Some(correct/data.size() as f32);
//    request.map_location = Some((abs_len, n_comp_regs as usize));
//    request
//}
//
////place abs len, comp_reg, penalize effective len
//pub fn ev_abs_comp_pen_eff_len(mut request: EvalResult, data: &TestDataSet) -> EvalResult {
//    let n_comp_regs = request.genome.n_calc_regs;
//    let abs_len = request.genome.instructions.len();
//
//    let mut correct = eval_program_corrects(&request.genome, data);
//    let penalty = (request.genome.get_effective_len(0) as f32)*0.75;
//    correct -= penalty;
//
//    request.genome.test_fit = Some(correct/data.size() as f32);
//    request.map_location = Some((abs_len, n_comp_regs as usize));
//    request
//}
//
//
////place effective feats, n_comp regs
//pub fn ev_eff_feat_comp(mut request: EvalResult, data: &TestDataSet) -> EvalResult {
//    let n_comp_regs = request.genome.n_calc_regs;
//    let n_feats_eff = request.genome.get_n_effective_feats(0);
//    let mut correct = eval_program_corrects(&request.genome, data);
//
//
//    request.genome.test_fit = Some(correct/data.size() as f32);
//    request.map_location = Some((n_feats_eff as usize, n_comp_regs as usize));
//    request
//}
//
////big square (n_feat, eff_len) small (n_comp, rand)
//pub fn ev_four(mut request: EvalResult, data: &TestDataSet) -> EvalResult {
//    let n_feats = request.genome.features.len();
//    let n_comp_regs = request.genome.n_calc_regs;
//
//    let feat_ind = get_mapped_index_step(n_feats, 2, 0,7);
//    let comp_ind = get_mapped_index_step(n_comp_regs as usize, 2, 0,7);
//    let eff_len = request.genome.get_effective_len(0);
//    let eff_len_ind = get_mapped_index_step(eff_len, 2, 0,7);
//
//    let rand: usize = thread_rng().gen_range(0,7);
//
//    let loc = (feat_ind*7 + comp_ind, eff_len_ind*7 + rand);
//
//    let correct = eval_program_corrects(&request.genome, data);
//
//    request.genome.test_fit = Some(correct/data.size() as f32);
//    request.map_location = Some(loc);
//    request
//}
//
//
////big square (n_feat, abs_len) small (n_comp, rand)
//pub fn ev_four_abs_len(mut request: EvalResult, data: &TestDataSet) -> EvalResult {
//    let n_feats = request.genome.features.len();
//    let n_comp_regs = request.genome.n_calc_regs;
//
//    let feat_ind = get_mapped_index_step(n_feats, 2, 0,7);
//    let comp_ind = get_mapped_index_step(n_comp_regs as usize, 2, 0,7);
//
//    let abs_len = request.genome.get_abs_len();
//    let abs_len_ind = get_mapped_index_step(abs_len, 3, 0,7);
//
//    let rand: usize = thread_rng().gen_range(0,7);
//
//    let loc = (feat_ind*7 + comp_ind, abs_len_ind*7 + rand);
//
//    let correct = eval_program_corrects(&request.genome, data);
//
//    request.genome.test_fit = Some(correct/data.size() as f32);
//    request.map_location = Some(loc);
//    request
//}
//
//
////big square (n_eff_feat, eff_len) small (n_eff_comp, rand)
//pub fn ev_four_effective_small(mut request: EvalResult, data: &TestDataSet) -> EvalResult {
//    let n_eff_feats = request.genome.get_n_effective_feats(0);
//    let n_comp_regs = request.genome.get_n_effective_comp_regs(0);
//
//    let feat_ind = get_mapped_index_step(n_eff_feats, 1, 3,7);
//    let comp_ind = get_mapped_index_step(n_comp_regs as usize, 1,0, 7);
//
//    let eff_len = request.genome.get_abs_len();
//    let eff_len_ind = get_mapped_index_step(eff_len, 1, 0,7);
//
//    let rand: usize = thread_rng().gen_range(0,7);
//
//    let loc = (feat_ind*7 + comp_ind, eff_len_ind*7 + rand);
//
//    let correct = eval_program_corrects(&request.genome, data);
//
//    request.genome.test_fit = Some(correct/data.size() as f32);
//    request.map_location = Some(loc);
//    request
//}
//
//
////big square (n_feat, abs_len) small (n_comp, rand)
//pub fn ev_four_abs_len_noise(mut request: EvalResult, data: &TestDataSet) -> EvalResult {
//    let n_feats = request.genome.features.len();
//    let n_comp_regs = request.genome.n_calc_regs;
//
//    let feat_ind = get_mapped_index_step(n_feats, 2, 0,7);
//    let comp_ind = get_mapped_index_step(n_comp_regs as usize, 2, 0,7);
//
//    let abs_len = request.genome.get_abs_len();
//    let abs_len_ind = get_mapped_index_step(abs_len, 3, 0,7);
//
//    let rand: usize = thread_rng().gen_range(0,7);
//
//    let loc = (feat_ind*7 + comp_ind, abs_len_ind*7 + rand);
//
//    let noise = thread_rng().gen_range(-(rand as f32), (rand+1) as f32);
//
//    let mut correct = eval_program_corrects(&request.genome, data);
//    correct += noise;
//
//
//    request.genome.test_fit = Some(correct/data.size() as f32);
//    request.map_location = Some(loc);
//    request
//}
//
//
////big square (n_feat, abs_len) small (n_comp, rand)
//pub fn ev_four_abs_len_pen(mut request: EvalResult, data: &TestDataSet) -> EvalResult {
//    let n_feats = request.genome.features.len();
//    let n_comp_regs = request.genome.n_calc_regs;
//
//    let feat_ind = get_mapped_index_step(n_feats, 2, 0,7);
//    let comp_ind = get_mapped_index_step(n_comp_regs as usize, 2, 0,7);
//
//    let abs_len = request.genome.get_abs_len();
//    let abs_len_ind = get_mapped_index_step(abs_len, 3, 0,7);
//
//    let rand: usize = thread_rng().gen_range(0,7);
//
//    let loc = (feat_ind*7 + comp_ind, abs_len_ind*7 + rand);
//
//    let penalty = match rand {
//        0 => 0,
//        1 => request.genome.get_n_effective_comp_regs(0),
//        2 => request.genome.n_calc_regs as usize,
//        3 => request.genome.get_abs_len(),
//        4 => request.genome.get_effective_len(0),
//        5 => request.genome.get_n_effective_feats(0),
//        6 => request.genome.features.len(),
//        _ => panic!("error shouldnt be here!")
//    };
//
//    let correct = eval_program_corrects(&request.genome, data) - penalty as f32;
//
//    request.genome.test_fit = Some(correct/data.size() as f32);
//    request.map_location = Some(loc);
//    request
//}
//
//
////pub fn ev_f_c_varpen1(mut request: Message, data: &TestDataSet) -> Message {
////    let period = 100_000.0/std::f64::consts::PI;
////    let min = -1.0;
////    let max = 1.0;
////
////    let ampli = (max - min)/2.0;
////    let mid = (max+min)/2.0;
////    let penalty = mid + ampli*(request.trial_no as f64/period).sin();
////
////
////    let n_comp_regs = request.genome.n_calc_regs;
////    let n_feats = request.genome.features.len();
////    let correct = eval_program_corrects(&request.genome, data) - penalty as f32;
////
////    request.genome.test_fit = Some(correct/data.size() as f32);
////    request.map_location = Some((n_feats as usize, n_comp_regs as usize));
////    request
////}
////
////
////pub fn ev_f_c_varpen2(mut request: Message, data: &TestDataSet) -> Message {
////    let period = 100_000.0/std::f64::consts::PI;
////    let min = -2.0;
////    let max = 2.0;
////
////    let ampli = (max - min)/2.0;
////    let mid = (max+min)/2.0;
////    let penalty = mid + ampli*(request.trial_no as f64/period).sin();
////
////
////    let n_comp_regs = request.genome.n_calc_regs;
////    let n_feats = request.genome.features.len();
////    let correct = eval_program_corrects(&request.genome, data) - penalty as f32;
////
////    request.genome.test_fit = Some(correct/data.size() as f32);
////    request.map_location = Some((n_feats as usize, n_comp_regs as usize));
////    request
////}
////
////
////pub fn ev_f_c_varpen3(mut request: Message, data: &TestDataSet) -> Message {
////    let period = 10_000.0/std::f64::consts::PI;
////    let min = -1.0;
////    let max = 1.0;
////
////    let ampli = (max - min)/2.0;
////    let mid = (max+min)/2.0;
////    let penalty = mid + ampli*(request.trial_no as f64/period).sin();
////
////
////    let n_comp_regs = request.genome.n_calc_regs;
////    let n_feats = request.genome.features.len();
////    let correct = eval_program_corrects(&request.genome, data) - penalty as f32;
////
////    request.genome.test_fit = Some(correct/data.size() as f32);
////    request.map_location = Some((n_feats as usize, n_comp_regs as usize));
////    request
////}
////
////
////pub fn ev_f_c_varpen4(mut request: Message, data: &TestDataSet) -> Message {
////    let period = 10_000.0/std::f64::consts::PI;
////    let min = -2.0;
////    let max = 2.0;
////
////    let ampli = (max - min)/2.0;
////    let mid = (max+min)/2.0;
////    let penalty = mid + ampli*(request.trial_no as f64/period).sin();
////
////
////    let n_comp_regs = request.genome.n_calc_regs;
////    let n_feats = request.genome.features.len();
////    let correct = eval_program_corrects(&request.genome, data) - penalty as f32;
////
////    request.genome.test_fit = Some(correct/data.size() as f32);
////    request.map_location = Some((n_feats as usize, n_comp_regs as usize));
////    request
////}
////
////
////pub fn ev_f_c_varpen5(mut request: Message, data: &TestDataSet) -> Message {
////    let period = 500_000.0/std::f64::consts::PI;
////    let min = -1.0;
////    let max = 1.0;
////
////    let ampli = (max - min)/2.0;
////    let mid = (max+min)/2.0;
////    let penalty = mid + ampli*(request.trial_no as f64/period).sin();
////
////
////    let n_comp_regs = request.genome.n_calc_regs;
////    let n_feats = request.genome.features.len();
////    let correct = eval_program_corrects(&request.genome, data) - penalty as f32;
////
////    request.genome.test_fit = Some(correct/data.size() as f32);
////    request.map_location = Some((n_feats as usize, n_comp_regs as usize));
////    request
////}
////
////
////pub fn ev_f_c_varpen6(mut request: Message, data: &TestDataSet) -> Message {
////    let period = 100_000.0/std::f64::consts::PI;
////    let min = 0.0;
////    let max = 2.0;
////
////    let ampli = (max - min)/2.0;
////    let mid = (max+min)/2.0;
////    let penalty = mid + ampli*(request.trial_no as f64/period).sin();
////
////
////    let n_comp_regs = request.genome.n_calc_regs;
////    let n_feats = request.genome.features.len();
////    let correct = eval_program_corrects(&request.genome, data) - penalty as f32;
////
////    request.genome.test_fit = Some(correct/data.size() as f32);
////    request.map_location = Some((n_feats as usize, n_comp_regs as usize));
////    request
////}
////
////
////pub fn ev_f_c_varpen7(mut request: Message, data: &TestDataSet) -> Message {
////    let period = 500_000.0/std::f64::consts::PI;
////    let min = 0.0;
////    let max = 3.0;
////
////    let ampli = (max - min)/2.0;
////    let mid = (max+min)/2.0;
////    let penalty = mid + ampli*(request.trial_no as f64/period).sin();
////
////
////    let n_comp_regs = request.genome.n_calc_regs;
////    let n_feats = request.genome.features.len();
////    let correct = eval_program_corrects(&request.genome, data) - penalty as f32;
////
////    request.genome.test_fit = Some(correct/data.size() as f32);
////    request.map_location = Some((n_feats as usize, n_comp_regs as usize));
////    request
////}
////
////
////pub fn ev_f_c_varpen8(mut request: Message, data: &TestDataSet) -> Message {
////    let period = 500_000.0/std::f64::consts::PI;
////    let min = -2.0;
////    let max = 4.0;
////
////    let ampli = (max - min)/2.0;
////    let mid = (max+min)/2.0;
////    let penalty = mid + ampli*(request.trial_no as f64/period).sin();
////
////
////    let n_comp_regs = request.genome.n_calc_regs;
////    let n_feats = request.genome.features.len();
////    let correct = eval_program_corrects(&request.genome, data) - penalty as f32;
////
////    request.genome.test_fit = Some(correct/data.size() as f32);
////    request.map_location = Some((n_feats as usize, n_comp_regs as usize));
////    request
////}
////
////
////
////pub fn ev_f_c_varpen9(mut request: Message, data: &TestDataSet) -> Message {
////    let min = -2.0;
////    let max = 2.0;
////
////    let n_comp_regs = request.genome.n_calc_regs;
////    let n_feats = request.genome.features.len();
////    let mut correct = eval_program_corrects(&request.genome, data);
////
////    let trial_no = request.trial_no;
////
////
////    if (trial_no > 6_000_000 && trial_no < 9_000_000) ||
////        (trial_no > 16_000_000 && trial_no < 19_000_000) {
////
////
////        let ampli = (max - min)/2.0;
////        let mid = (max+min)/2.0;
////
////        let period = 2_000_000.0;
////        let value = (trial_no as f64)*2.0*std::f64::consts::PI/period;
////        let penalty = mid + ampli*value.sin();
////
////        correct -= penalty as f32;
////    }
////
////    request.genome.test_fit = Some(correct/data.size() as f32);
////    request.map_location = Some((n_feats as usize, n_comp_regs as usize));
////    request
////}



fn get_mapped_index(n: usize) -> usize{
    let mut top =0;
    let mut step = 1;
    let mut incr_step = false;
    for i in 0..10{
        top += step;
//        println!("top {}", top);
        if n <=  top{return i}
        else if incr_step {step +=2; }//println!("i is {} step is now {}", i,step); }
        incr_step = !incr_step; //incr every second
    }
    10
}


fn get_mapped_index_step(n: usize, step: usize, start: usize, max: usize) -> usize{
    let mut top =start;
    for i in 0..max{
        top += step;
        if n <=  top{return i}
    }
    max
}


fn reduce3to2dim(x: usize, y: usize, z: usize) -> (usize, usize){
    let x_new = if y < 5 {y*10 + x } else{ (y-5)*10 + x };
    let y_new = if y < 5 {z } else if z < 50 { 50 - z } else {0}; //should not be else
    //but didnt want prog to crash in the very rare case z>= 50
    (x_new, y_new)
}

pub fn eval_program_corrects(genome: &super::prog::Program, data: &DataSet) -> f32 {

    let n_feats = genome.features.len();

    let mut correct = 0.0f32;

    for (sample_i, record) in data.record_iter().enumerate() {

        let mut regs = [0.0f32; params::params::MAX_REGS];

        let mut reg_val = 0.1;
        for reg in regs.iter_mut() { //semi random initilize regs
            *reg = reg_val;
            reg_val = -(reg_val + 0.05);
        }

        for (i, feature) in genome.features.iter().enumerate() { //load features
            regs[params::params::MAX_REGS - 1 - i] = record.features[*feature as usize]
        }

        let prog_output = genome.execute_important_instructions(regs, &genome.get_important_instrs(0));
//        let prog_output = genome.execute_instructions(regs);

        let prog_result = prog_output >= 0.0;
        if prog_result == record.class {correct += 1.0;}
    }
    correct as f32
}


pub fn eval_program_cv(genome: &super::prog::Program, data: &ValidationSet) -> f32 {
    let correct = eval_program_corrects(genome, data);
    correct/ data.size() as f32
}