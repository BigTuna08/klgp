use super::*;
use crate::dataset::KDataSet;

pub const MAX_REGS: usize = 128;
pub const N_OPS: i32 = 6;

pub fn float_to_pred(f: f32)-> bool // true iff f >= 0.0
{
    0.0 < f
}

pub fn score_prog(prog_data: &[i32], data: &KDataSet, from:usize, to:usize) -> f32{
    let correct = (from..to)
            .filter(|i| data.y[*i] ==
                float_to_pred(eval_prog_sample(prog_data, &data.x[*i])) )
            .count();

    correct as f32 / ((to-from) as f32)
}


pub fn basic_cmp_progs(p1: &Program, p2: &Program) -> Ordering{
    if p1.fit == p2.fit{
        if p1.pheno.complexity == p2.pheno.complexity{
            Ordering::Equal
        }
        else {p1.pheno.complexity.cmp(&p2.pheno.complexity)}
    }
    else { match p1.fit.partial_cmp(&p2.fit){
        Some(val) => val,
        None => panic!("Invalid comparison! {:?}, {:?}", &p1.fit, &p2.fit)
    } }
}

pub fn basic_pf_map(pheno: &FLPheno, ee: &EvalEnv) -> f32{
    agreement_score(&pheno.preds, &ee.y[0..ee.tr_end])
}

pub fn pf_map_pen_complex(pheno: &FLPheno, ee: &EvalEnv) -> f32{
    let pen = 0.005;
    agreement_score(&pheno.preds, &ee.y[0..ee.tr_end]) - pheno.complexity as f32 * pen
}


pub fn agreement_score<T: std::cmp::PartialEq>(v1: &[T], v2: &[T]) -> f32{
    assert_eq!(v1.len(), v2.len());

    let mut agree = 0;
    for (i,j) in v1.iter().zip(v2){
        if *i==*j {agree += 1;}
    }
    agree as f32 / v1.len() as f32
}

pub fn basic_lgp_map(geno: &VLGeno, de: &DevelEnv) -> FLPheno{
    let mut pvec = [false; FLP_SIZE];

    for i in 0..de.tr_end{
        pvec[i] = 0.0 < eval_prog_sample(&geno.raw_seq, &de.x[i]);
    }

    FLPheno{preds:pvec, complexity:geno.len()}
}


pub fn eval_prog_sample(prog_data: &[i32], feats: &[f32] ) -> f32 {
    let mut regs = PROG_REG.clone();

    let mut buffer = Vec::with_capacity(8);
    let mut current_targ = 0;
    let flen = feats.len();
    assert_eq!(flen, data_params::NFEAT);

    for i in prog_data.iter() {
        // println!("{}", i);
        if *i < N_OPS {
            // do op
            regs[current_targ] = match *i {
                0 => { // add
                    buffer.drain(..).sum()
                }
                1 => { // subt
                    // println!("\nsubtr with buffer = {:?}", &buffer);
                    // println!("regs before = {:?}", &regs[0..10]);
                    buffer.drain(..).fold(0.0, |acc, x: f32| acc - x)
                }
                2 => { // mult
                    buffer.drain(..).fold(0.0, |acc, x| acc * x)
                }
                3 => { // pdiv
                    buffer.drain(..)
                        .fold(0.0, |acc: f32, x: f32| if x == 0.0 { acc } else { acc / x })
                }
                4 => { // like if > (stores 0/1)
                    if buffer.len() == 0 { 0.0 }
                    else {
                        let val = buffer.swap_remove(0);
                        if val > buffer.drain(..).sum() {
                            1.0
                        } else { 0.0 }
                    }
                }
                5 => { // like if > (v2) (stores 0/1)
                    let val = match buffer.len() {
                        0 => 0.0,
                        1 => if buffer[0] > 0.0 {1.0} else {0.0},
                        2 => if buffer[0] > 0.0 {1.0} else {buffer[1]},
                        3 => if buffer[0] > 0.0 {buffer[2]} else { buffer[1] }
                        _ => if buffer[0] > buffer[3] {buffer[2]} else { buffer[1] }
                        // any items after the 4th are ignored
                    };
                    buffer.drain(..);
                    val
                }
                _ => panic!("Err in execution, unknown op. Code={}", i)
            };
            current_targ += 1;
            // if current_targ >= MAX_REGS {break} // limits # of instr
            if current_targ >= MAX_REGS {current_targ=0} // loops through comp regs

            // println!("regs after = {:?}", &regs[0..10]);
        }
        else if *i < N_OPS + flen as i32 {
            buffer.push(feats[(*i - N_OPS) as usize])
        }
        else { // add value from reg to be processed.
            buffer.push(regs[(*i - N_OPS - flen as i32) as usize])
        }
    }
    // println!("Done with cur tar = {}\n will return {}", current_targ, match current_targ {
    //     0 => 0.0,
    //     _ => regs[current_targ - 1]
    // });

    match current_targ {
        0 => 0.0,
        _ => regs[current_targ - 1]
    }
}





pub const PROG_REG: &[f32; MAX_REGS] = &[0.0,
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
    0.03846154,
    -0.037037037,
    -0.035714287,
    0.03448276,
    0.033333335,
    -0.032258064,
    -0.03125,
    0.030303031,
    0.029411765,
    -0.028571429,
    -0.027777778,
    0.027027028,
    0.02631579,
    -0.025641026,
    -0.025,
    0.024390243,
    0.023809524,
    -0.023255814,
    -0.022727273,
    0.022222223,
    0.02173913,
    -0.021276595,
    -0.020833334,
    0.020408163,
    0.02,
    -0.019607844,
    -0.01923077,
    0.018867925,
    0.018518519,
    -0.018181818,
    -0.017857144,
    0.01754386,
    0.01724138,
    -0.016949153,
    -0.016666668,
    0.016393442,
    0.016129032,
    -0.015873017,
    -0.015625,
    0.015384615,
    0.015151516,
    -0.014925373,
    -0.014705882,
    0.014492754,
    0.014285714,
    -0.014084507,
    -0.013888889,
    0.01369863,
    0.013513514,
    -0.013333334,
    -0.013157895,
    0.012987013,
    0.012820513,
    -0.012658228,
    -0.0125,
    0.012345679,
    0.0121951215,
    -0.012048192,
    -0.011904762,
    0.011764706,
    0.011627907,
    -0.011494253,
    -0.011363637,
    0.011235955,
    0.011111111,
    -0.010989011,
    -0.010869565,
    0.010752688,
    0.010638298,
    -0.010526316,
    -0.010416667,
    0.010309278,
    0.010204081,
    -0.01010101,
    -0.01,
    0.00990099,
    0.009803922,
    -0.009708738,
    -0.009615385,
    0.00952381,
    0.009433962,
    -0.009345794,
    -0.009259259,
    0.0091743115,
    0.009090909,
    -0.009009009,
    -0.008928572,
    0.0088495575,
    0.00877193,
    -0.008695652,
    -0.00862069,
    0.008547009,
    0.008474576,
    -0.008403362,
    -0.008333334,
    0.008264462,
    0.008196721,
    -0.008130081,
    -0.008064516,
    0.008,
    0.007936508,
    -0.007874016,
    -0.0078125,
];