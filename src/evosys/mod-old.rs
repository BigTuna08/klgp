pub mod gp_maps;
pub mod mutators;
pub mod helpers;

use std::iter::{Iterator, IntoIterator};
use rand::{Rng};
use rand::distributions::Standard;
use rand::prelude::Distribution;
use std::cmp::{Ordering, max};
use std::collections::HashMap;
use toml::Value;
use toml::value::Array;

use std::time::Instant;

use crate::dataset::params as data_params;
use crate::dataset;


//////////////////////////////////////////////////////////////

use gp_maps::basic_lgp_map as gpmap;
// use gp_maps::basic_pf_map as pfmap;
use gp_maps::pf_map_pen_complex as pfmap;
use gp_maps::basic_cmp_progs as compare_progs;

use mutators::mutate;


/////////////////////           Set Constants Here!          ////////////////////////////////////
pub const FLG_SIZE: usize = 10; // fixed length geno
pub const FLP_SIZE: usize = data_params::NSAMP_TRAIN; // fixed length geno

pub const GENE_MIN: GeneType = 0;
pub const GENE_MAX: GeneType = gp_maps::N_OPS + data_params::NFEAT as i32+ gp_maps::MAX_REGS as i32;

/////////////////////           Set Types Here!          ////////////////////////////////////
type GeneType = i32;
type Genotype = VLGeno;
type Phenotype = FLPheno;
type FitType = f32;
type Program = KProg;

type PopType = BasicPop;
type EAType = BasicEA;


type DevelEnv = dataset::KDataSet;
type EvalEnv = dataset::KDataSet;

type GPMapF = dyn Fn(&Genotype, &DevelEnv) -> Phenotype;


////////////////////        Type definitions below        /////////////////////////////////////

pub struct BasicEA{
    pub pop: PopType,
    pub eval_env: EvalEnv,
    // pub devel_env: DevelEnv,
}

impl BasicEA{

}

pub trait EA{
    fn run(&mut self);
    fn new(args: &toml::Value) -> Self;
    fn cv(&self);
}

impl EA for BasicEA{
    fn run(&mut self) {
        let n_evals = 100_000_000;
        let cv_interval = 50_000;

        let start_time = Instant::now();
        let mut last_time = Instant::now();

        for eval_i in 0..n_evals{
            let geno = self.pop.select_prog();

            let newgeno = mutate(&geno.geno);
            let pheno = gpmap(&newgeno, &self.eval_env);
            let fit = pfmap(&pheno, &self.eval_env);
            self.pop.try_add(Program{geno:newgeno, pheno, fit});

            if eval_i % cv_interval == 0 && eval_i > 0 {

                println!("After {} evals", eval_i);
                time_update(cv_interval, &last_time);
                self.cv();

                last_time = Instant::now();
            }
        }

        println!("Overall time");
        time_update(n_evals, &start_time);
    }


    fn new(args: &toml::Value) -> BasicEA{

        let data = //Box::new(
            dataset::KDataSet::load_data("C:/Users/BigTuna/Dropbox/par_lgp/penndata/oa.csv",
                                                         data_params::NSAMP_TRAIN,
                                         data_params::NSAMP_CV);
        // );
        let pop = PopType::new(args, &data, &data);
        BasicEA{pop, eval_env:data}
    }


    fn cv(&self){
        let scores_tr: Vec<f32> = self.pop.progs.iter().map(
            |prog| prog.fit
        ).collect();

        let scores_cv: Vec<f32> = self.pop.progs.iter().map(
            |prog| gp_maps::score_prog(&prog.geno.show_geno(),
                                                    &self.eval_env,
                                                    self.eval_env.tr_end,
                                                    self.eval_env.cv_end)
        ).collect();

        let ave_len = self.pop.progs.iter().map(
            |prog| prog.pheno.complexity
        ).sum::<usize>() as f64 / self.pop.progs.len() as f64;

        println!("On training, max fit = {} , min is = {}",
                 my_max(scores_tr.as_ref()), my_min(scores_tr.as_ref()));
        println!("On cv, max fit = {} , min is = {}\n",
                 my_max(scores_cv.as_ref()), my_min(scores_cv.as_ref()));
        println!("Ave complexity is {:?}", ave_len);

    }
}

pub fn time_update(nevals: i32, timer: &Instant) {
    println!("Evaluated {} programs in {:?} secs ( {:?} micro secs per eval)",
             nevals,
             timer.elapsed().as_millis() as f64 / 1000.0,
             (timer.elapsed().as_micros() as f64 / nevals as f64 ));
}

pub fn my_max(vals: &[f32]) -> f32{
    vals.iter().fold(f32::MIN, |acc, x| x.max(acc))
}

pub fn my_min(vals: &[f32]) -> f32{
        vals.iter().fold(f32::MAX, |acc, x| x.min(acc))
}

pub struct KGPMap{
    env: GPMapF
}

pub struct KPFMap{
    env: EvalEnv
}

pub struct BasicPop{
    progs: Vec<Program>,
    select_i: usize,
    min_fit: FitType,
}



pub trait Population{
    fn new(args: &toml::Value, devel_env: &DevelEnv, eval_env: &EvalEnv) -> Self;
    fn progs(&self) -> &[Program];
    fn try_add(&mut self, prog: Program) -> bool;
    fn select_prog(&mut self) -> &Program;
}



impl Population for BasicPop{
    fn progs(&self) -> &[Program]{
        self.progs.as_slice()
    }

    fn try_add(&mut self, prog: Program) -> bool{
        let locs = [1usize, 7, 14, 16, 29, 35, 44, 47, 52, 59];
        let loc: usize = locs.iter().enumerate().map(
            |(i,x)| if prog.pheno.preds[*x] {2usize.pow(i as u32)}
                                    else {0}
        ).sum();

        let replace = match compare_progs(&prog, &self.progs[loc]) {
            Ordering::Greater => { true },
            Ordering::Less => {false},
            Ordering::Equal => {rand::thread_rng().gen::<bool>()}
        };

        if replace {self.progs[loc] = prog}
        replace
    }



    fn select_prog(&mut self) -> &Program{
        self.select_i = (self.select_i + 1) % self.progs.len();
        if self.progs[self.select_i].fit > self.min_fit{
            &self.progs[self.select_i]
        }
        else{self.select_prog()}
    }

    fn new(args: &toml::Value, devel_env: &DevelEnv, eval_env: &EvalEnv) -> BasicPop{
        let mut pop_size = extract_val_vec("pop_size", args);//args.get("pop_size");
        let pop_size = pop_size.pop().expect("pop_size not found!")
            .as_integer().expect("pop size must be int!") as usize;

        let min_fit = extract_val_vec("min_fit", args).pop().expect("min_fit not found")
            .as_float().expect("min fit must be float!") as f32;

        let init_pop_size = extract_val_vec("init_pop_size", args).pop().expect("init_pop_size not found")
            .as_integer().expect("init_pop_size must be int!") as usize;

        assert!(pop_size <= init_pop_size);

        let mut progs = Vec::with_capacity(pop_size);

        for _ in 0..pop_size{
            let geno = Genotype::new();
            let pheno = gpmap(&geno, devel_env);
            let fit = pfmap(&pheno, eval_env);
            progs.push(Program{geno, pheno, fit});
        }

        if pop_size==init_pop_size{
            BasicPop{
                progs, min_fit, select_i:0
            }
        }
        else{
            let mut pop = BasicPop{
                progs, min_fit, select_i:0
            };
            for _ in pop_size..init_pop_size{
                let geno = Genotype::new();
                let pheno = gpmap(&geno, devel_env);
                let fit = pfmap(&pheno, eval_env);
                pop.try_add(Program{geno, pheno, fit});
            }
            pop
        }
    }
}

// pub fn extract_val_vec


pub fn extract_val_vec(name: &str, table: &Value) -> Vec<Value> {
    match table.get(name) {
        Some(x) => match x {
            Value::Array(x) => x.clone(),
            _ => panic!("bad type! expected array for {} found \n{:?}", name, x)
        }
        None => panic!("key not found!\n Err getting {} from \n{:?}", name, table)
    }
}







pub struct KProg{
    geno: Genotype,
    pheno: Phenotype,
    fit: FitType,
}

pub trait Evolvalbe{
    fn new_random() -> Self;
    fn new_mutated(genome: &Genotype) -> Self;
    fn new_crossover(genome1: &Genotype, genome2: &Genotype)-> Self;
}


pub const MIN_IN_L: usize = 1;
pub const MAX_IN_L: usize = 5;

pub const MIN_INS: usize = 1;
pub const MAX_INS: usize = 10;

pub const ACT_REGS: i32 = 5;
//
// impl Evolvalbe for KProg{
//     fn new_random() -> KProg{
//         let mut rng = rand::thread_rng();
//         let len = rng.gen_range(MIN_INS, MAX_INS);
//         let mut new_geno = Vec::with_capacity(len*MAX_IN_L);
//         let mut max_act_reg = ACT_REGS;
//
//         for _ in 0..rng.gen_range(MIN_INS, MAX_INS){
//             new_geno.push(rng.gen_range(0, gp_maps::N_OPS)); // OP
//
//             for _ in 0..rng.gen_range(MIN_IN_L, MAX_IN_L){
//                 let max_val = gp_maps::N_OPS + data_params::NFEAT + max_act_reg;
//                 let arg = rng.gen_range(gp_maps::N_OPS, max_act_reg);
//                 if arg == max_act_reg-1 {max_act_reg +=1}
//                 new_geno.push(arg); // args
//             }
//         }
//
//         KProg
//     }
//
//     fn new_mutated(genome: &Genotype) -> KProg{
//
//     }
//
//     fn new_crossover(genome1: &Genotype, genome2: &Genotype)-> KProg{
//
//     }
// }

pub struct VLGeno{
    raw_seq: Vec<GeneType>,
}

pub struct FLGeno{
    raw_seq: [GeneType; FLG_SIZE],
}

pub struct FLPheno{
    preds: [bool; FLP_SIZE],
    complexity: usize
}

impl FLPheno{
    pub fn new_blank() -> FLPheno{
        FLPheno{preds:[false; FLP_SIZE], complexity:0}
    }
}

pub enum Genome{
    VL(VLGeno),
    FL(FLGeno)
}

//
// pub fn mutate_seq()


// pub enum Genome<T>{
//     VLGeno(T),
//     FLGeno(T),
// }
//
// pub fn testfn<T>(input: Genome<T>){
//     match input {
//         VLGeno(genes) => {
//             println!("vl geno! {:?}", genes)
//         }
//         FLGeno(genes) => {
//             println!("fl geno! {:?}", genes)
//         }
//     }
// }

pub trait Mutatable {
    fn show_geno(&self) -> &[GeneType];
    fn show_mutable_geno(&mut self) -> &mut [GeneType];
    fn copy(&self) -> Self;
    fn len(&self) -> usize;
    fn new() -> Self;
}

pub trait GPMap {
    fn develop(&self, geno: Genotype) -> Phenotype;
}

pub trait PFMap {
    fn evaluate(&self, pheno: Phenotype) -> FitType;
}

impl Mutatable for VLGeno{
    fn show_geno(&self) -> &[GeneType]{
        self.raw_seq.as_slice()
    }

    fn show_mutable_geno(&mut self) -> &mut [GeneType]{
        self.raw_seq.as_mut()
    }

    fn copy(&self) -> VLGeno{
        VLGeno{
            raw_seq:self.raw_seq.clone()
        }
    }

    fn len(&self) -> usize{
        self.raw_seq.len()
    }

    fn new() -> VLGeno{
        let mut rng = rand::thread_rng();
        let len = rng.gen_range(MIN_INS, MAX_INS);
        let mut new_geno = Vec::with_capacity(len*MAX_IN_L);
        let mut max_act_reg = ACT_REGS;

        for _ in 0..rng.gen_range(MIN_INS, MAX_INS){
            new_geno.push(rng.gen_range(0, gp_maps::N_OPS as i32)); // OP

            for _ in 0..rng.gen_range(MIN_IN_L, MAX_IN_L){
                let max_val =gp_maps::N_OPS + data_params::NFEAT as i32 + max_act_reg;
                let arg = rng.gen_range(gp_maps::N_OPS, max_val) as i32;
                if arg == max_val-1 {max_act_reg +=1}
                new_geno.push(arg); // args
            }
        }

        new_geno.shrink_to_fit();
        VLGeno{raw_seq:new_geno}
    }
}
//
// impl Mutatable for FLGeno{
//     fn show_geno(&self) -> &[GeneType]{
//         &self.raw_seq
//     }
//
//     fn show_mutable_geno(&mut self) -> &mut [GeneType]{
//         self.raw_seq.as_mut()
//     }
//
//     fn copy(&self) -> Self{
//         FLGeno{
//             raw_seq:self.raw_seq.clone()
//         }
//     }
//
//     fn len(&self) -> usize{
//         self.raw_seq.len()
//     }
// }



pub fn new_mutated(geno: impl Mutatable) -> impl Mutatable{
    let mut newg = geno.copy();
    let mut rng  = rand::thread_rng();
    let (loc, val) = (rng.gen_range(0, newg.len()) ,
                      rng.gen::<GeneType>());

    newg.show_mutable_geno()[loc] = val;
    newg
}

// impl<T> Mutatable<T> for VLGeno<T>{
//     fn show_geno(&self) -> &[T] {
//         self.raw_seq.as_slice()
//     }
// }

// impl Mutatable<i32> for VLGeno<i32>{
//     fn show_geno(&self) -> &[i32] {
//         self.raw_seq.as_slice()
//     }
// }

// pub fn rand_mutate<>(geno: impl Mutatable<T>) -> impl Mutatable<T>
//     where
//         Standard: Distribution<T>{
//     let g = geno.show_geno();
//     println!("Geno len {:?}, geno: {:?}", g.len(), &g.len());
//     let newg = g.iter().map(|item| {
//         if rand::thread_rng().gen_bool(0.2f64) { item }
//         else { &rand::thread_rng().gen::<T>() }
//     }).collect();
//
//     Genotype{
//         raw_seq:newg
//     }
// }

pub fn evohi(){
    println!("evo hi");
    // let gp = gpmap(0);
    // println!("evo by {}", gp);
    // rustlearn::datasets::iris::load_data()
}