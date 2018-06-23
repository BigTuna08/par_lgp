use params;

pub const PROG_REG: &[f32; params::params::MAX_REGS] = &[0.0,
    0.5,
    -0.33333334,
    -0.25,
    0.2,
    0.16666667,
    -0.14285715,
    -0.125,
    0.11111111,
    0.1,
    -0.09090909,
    -0.083333336,
    0.07692308,
    0.071428575,
    -0.06666667,
    -0.0625,
    0.05882353,
    0.055555556,
    -0.05263158,
    -0.05,
    0.04761905,
    0.045454547,
    -0.04347826,
    -0.041666668,
    0.04,
];

pub fn make_regs(){
    let mut initial_regs = [0.0f32; params::params::MAX_REGS];


    let mut negative = false;
    let mut switch_sign_next = true;

    for i in 1..params::params::MAX_REGS as usize{

        let val = match negative {
            true => -((i+1) as f32),
            false => (i+1) as f32,
        };
        initial_regs[i] = 1.0/val;

        if switch_sign_next{ negative = !negative }

        switch_sign_next = !switch_sign_next //switch every other
    }
    print!("pub const PROG_REG: &[f32; params::params::MAX_REGS] = &[");
    for n in initial_regs.iter(){
        println!("{},",n);
    }
    print!("]");
}