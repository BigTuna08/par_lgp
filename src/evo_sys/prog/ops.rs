use params;

pub static OPS: &'static [fn(f32, f32) -> f32] =
    &[add, subt, mult, pdiv, pow, log, sig, big];

pub static OPS_NAMES: &'static [&'static str] =
    &["add", "subt", "mult", "pdiv", "pow", "log", "bip" , "big"];


//pub static OPS_NAMES: &'static [(&'static str, fn(f32, f32) -> f32)] =
//    &[("add", add), ("subt", subt), ("mult", mult), ("pdiv", pdiv),
//        ("pow", pow), ("log", log), ("bip", bip), ("big", big)];

pub fn add(opr1: f32, opr2: f32) -> f32 {
    opr1 + opr2
}

pub fn subt(opr1: f32, opr2: f32) -> f32 {
    opr1 - opr2
}

pub fn mult(opr1: f32, opr2: f32) -> f32 {
    opr1 * opr2
}

pub fn pdiv(opr1: f32, opr2: f32) -> f32 {
    if opr2.abs() > params::params::EPS {opr1/opr2}
    else { opr1 } //protect if opr2 ~= 0
}

pub fn pow(opr1: f32, opr2: f32) -> f32 {
    opr1.powf(opr2)
}

pub fn log(opr1: f32, opr2: f32) -> f32 {
    opr1.log(opr2)
}

//branch if opr1 positive, ignore opr2
pub fn bip(opr1: f32, _opr2: f32) -> f32 {
    if opr1 > 0.0 {1.0} //branch
    else {-1.0} //dont
}

//skip if greater
pub fn sig(opr1: f32, opr2: f32) -> f32 {
    if opr1 > opr2 {1.0} //branch
        else {-1.0} //dont
}

//branch if greater
pub fn big(opr1: f32, opr2: f32) -> f32 {
    if opr1 > opr2 {1.0} //branch
    else {-1.0} //dont
}