pub mod gp_maps;
pub mod mutators;

pub mod prog;
pub mod pop;
pub mod ea;

pub mod helpers;



use std::iter::{Iterator, IntoIterator};
use rand::{Rng};
use rand::distributions::Standard;
use rand::prelude::Distribution;
use std::cmp::{Ordering};
use std::collections::HashMap;
use toml::Value;
use toml::value::{Array, Table};


use std::time::Instant;

use crate::dataset::params as data_params;
use crate::dataset;
use helpers::{time_update, my_max, my_min, extract_val_vec, from_string_vec};

//////////////////////////////////////////////////////////////

use gp_maps::basic_lgp_map as gpmap;
// use gp_maps::basic_pf_map as pfmap;
use gp_maps::pf_map_pen_complex as pfmap;
use gp_maps::basic_cmp_progs as compare_progs;

use mutators::mutate;
use crate::evosys::helpers::{extract_string_vec, extract_typed_vec};


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

type ProgDefaults = KProgDefaults;

type CVResult = f32;

type DevelEnv = dataset::KDataSet;
type EvalEnv = dataset::KDataSet;



////////////////////        Type definitions below        /////////////////////////////////////

#[derive(Copy, Clone, Debug)]
pub struct KProgDefaults{
    pub min_il: usize,
    pub max_il: usize,
    pub min_ins: usize,
    pub max_ins: usize,
    pub eff_regs: i32,
}


// pub struct KPopConfig{
//     pub pop_size: usize,
//     pub init_pop_size: usize,
//     pub min_fit: FitType,
//     pub prog_defs: ProgDefaults,
// }

#[derive(Debug)]
pub struct KPopConfig{
    pub pop_defs: PConfig,
    pub prog_defs: ProgDefaults,
}


// for loc in extract_typed_vec::<String>("data_locs", args){
//     for min_fit in extract_typed_vec::<f32>("min_fit", args){
//         for n_evals in extract_typed_vec::<usize>("n_evals", args){
//             for pop_size in extract_typed_vec::<usize>("pop_size", args) {
//                 for init_pop_size in extract_typed_vec::<usize>("init_pop_size", args){
pub struct EARunner{
    pub locs: Vec<String>,
    pub min_fits: Vec<f32>,
    pub n_evals: Vec<usize>,
    pub pop_sizes: Vec<usize>,
    pub init_pop_sizes: Vec<usize>,
    pub mutation_methods: Vec<usize>,
    pub compare_methods: Vec<usize>,
    pub current_i: usize,
}

#[derive(Debug)]
pub struct PConfig{
    pub loc: String,
    pub min_fit: f32,
    pub n_eval: usize,
    pub pop_size: usize,
    pub init_pop_size: usize,
    pub mutation_method: usize,
    pub compare_method: usize,
}



impl EARunner{
    pub fn from_table(args: &Value) -> EARunner{
        let locs = extract_typed_vec::<String>("data_locs", args);
        let min_fits = extract_typed_vec::<f32>("min_fit", args);
        let n_evals = extract_typed_vec::<usize>("n_evals", args);
        let pop_sizes = extract_typed_vec::<usize>("pop_size", args);
        let init_pop_sizes = extract_typed_vec::<usize>("init_pop_size", args);
        let mutation_methods = extract_typed_vec::<usize>("mutation_methods", args);
        let compare_methods = extract_typed_vec::<usize>("compare_methods", args);

        println!("locs!\n{:?}\n{}", &locs[0], locs.len());

        EARunner{
            locs,
            min_fits,
            n_evals,
            pop_sizes,
            init_pop_sizes,
            mutation_methods,
            compare_methods,
            current_i:0,
        }
    }


    fn next_config(&mut self) -> Option<PConfig>{
        let i = self.current_i;
        self.current_i += 1;
        let d0 = self.compare_methods.len();
        let d1 = self.mutation_methods.len();
        let d2 = self.init_pop_sizes.len();
        let d3 = self.pop_sizes.len();
        let d4 = self.n_evals.len();
        let d5 = self.min_fits.len();
        let d6 = self.locs.len();

        if i == d0*d1*d2*d3*d4*d5*d6 {return None}

        let i0 = (i.div_euclid(1) )% d0;
        let i1 = (i.div_euclid(d0) )% d1;
        let i2 = (i.div_euclid(d0*d1) )% d2;
        let i3 = (i.div_euclid(d0*d1*d2) )% d3;
        let i4 = (i.div_euclid(d0*d1*d2*d3) )% d4;
        let i5 = (i.div_euclid(d0*d1*d2*d3*d4) )% d5;
        let i6 = (i.div_euclid(d0*d1*d2*d3*d4*d5) )% d6;

        let compare_method = self.compare_methods[i0];
        let mutation_method = self.mutation_methods[i1];
        let init_pop_size = self.init_pop_sizes[i2];
        let pop_size = self.pop_sizes[i3];
        let n_eval = self.n_evals[i4];
        let min_fit = self.min_fits[i5];
        let loc = self.locs[i6].clone();

        //this fn was generated in python
        Some(PConfig{
            compare_method,
            mutation_method,
            init_pop_size,
            pop_size,
            n_eval,
            min_fit,
            loc,
        })

    }

}

pub struct BasicEA{
    pub run_config: EARunner,
    pub prog_config: KProgDefaults,
    // pub current_config: usize,
    pub current_iter: usize,
    pub max_iter: usize,
    pub out_folder: String,
    pub log_freq: usize,
}


pub trait EA{
    fn new(args: toml::Value) -> Self;
    fn run(&mut self)->i32;
    // fn run(&mut self, pop: &mut impl Population)->i32;
    fn run_next(&mut self, config: PConfig) -> i32;
    fn cv(&self, pop: &impl Population, eval_env: &EvalEnv)-> CVResult;
}


pub struct BasicPop{
    progs: Vec<Program>,
    prog_defs: ProgDefaults,
    select_i: usize,
    pop_size: usize,
    init_size: usize,
    min_fit: FitType,
}


pub trait Population{
    fn new(args: KPopConfig) -> Self;
    fn init(&mut self, devel_env: &DevelEnv, eval_env: &EvalEnv);
    fn progs(&self) -> &[Program];
    fn try_add(&mut self, prog: Program) -> bool;
    fn next_new(&mut self) -> Genotype;
    fn prog_defs(&self) -> &ProgDefaults;
}


pub struct KProg{
    geno: Genotype,
    pheno: Phenotype,
    fit: FitType,
}

// pub trait Evolvalbe{
//     fn new_random() -> Self;
//     fn new_mutated(genome: &Genotype) -> Self;
//     fn new_crossover(genome1: &Genotype, genome2: &Genotype)-> Self;
// }

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



pub trait Mutatable {
    fn show_geno(&self) -> &[GeneType];
    fn show_mutable_geno(&mut self) -> &mut [GeneType];
    fn copy(&self) -> Self;
    fn len(&self) -> usize;
    fn new(defs: &ProgDefaults) -> Self;
}





pub fn evohi(){
    println!("evo hi");
}

//
// pub struct KGPMap{
//     env: GPMapF
// }
//
// pub struct KPFMap{
//     env: EvalEnv
// }
//
// pub trait GPMap {
//     fn develop(&self, geno: Genotype) -> Phenotype;
// }
//
// pub trait PFMap {
//     fn evaluate(&self, pheno: Phenotype) -> FitType;
// }
//
// pub fn new_mutated(geno: impl Mutatable) -> impl Mutatable{
//     let mut newg = geno.copy();
//     let mut rng  = rand::thread_rng();
//     let (loc, val) = (rng.gen_range(0, newg.len()) ,
//                       rng.gen::<GeneType>());
//
//     newg.show_mutable_geno()[loc] = val;
//     newg
// }