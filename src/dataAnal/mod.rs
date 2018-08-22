

use dataMgmt::{DataSet, metabolites};
// m is for metabolite
// v is for value (floating point number)


// ratio of samples with m1 < m2
pub fn get_comparison_ratio(m1: &str, m2: &str, data: &DataSet) -> f32{
    let i1 = metabolites::get_metabolite_index(m1);
    let i2 = metabolites::get_metabolite_index(m2);

    let mut m1_less_count = 0.0;
    let mut total_count = 0.0;
    for record in data.record_iter(){
        if record.features[i1] < record.features[i2] {
            m1_less_count += 1.0;
        }
        total_count += 1.0;
    }
    return m1_less_count/total_count;
}


// ratio of samples with m1 < m2
pub fn get_conditional_comparison_ratio(m1: &str, m2: &str, data: &DataSet, case: bool) -> f32{
    let i1 = metabolites::get_metabolite_index(m1);
    let i2 = metabolites::get_metabolite_index(m2);

    let mut m1_less_count = 0.0;
    let mut total_count = 0.0;
    for record in data.record_iter(){
        if record.class == case{
            if record.features[i1] < record.features[i2] {
                m1_less_count += 1.0;
            }
            total_count += 1.0;
        }

    }
    return m1_less_count/total_count;
}


// ratio of samples with m1 < v
pub fn get_less_than_ratio(m1: &str, v: f32, data: &DataSet) -> f32{
    let i1 = metabolites::get_metabolite_index(m1);

    let mut m1_less_count = 0.0;
    let mut total_count = 0.0;
    for record in data.record_iter(){
        if record.features[i1] < v {
            m1_less_count += 1.0;
        }
        total_count += 1.0;
    }
    return m1_less_count/total_count;
}


// ratio of samples with m1 < v
pub fn get_greater_than_ratio(m1: &str, v: f32, data: &DataSet) -> f32{
    let i1 = metabolites::get_metabolite_index(m1);

    let mut m1_greater_count = 0.0;
    let mut total_count = 0.0;
    for record in data.record_iter(){
        if record.features[i1] > v {
            m1_greater_count += 1.0;
        }
        total_count += 1.0;
    }
    return m1_greater_count/total_count;
}