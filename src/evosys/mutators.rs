use super::*;
use gp_maps::{N_OPS, MAX_REGS};
use data_params::NFEAT;

impl Genotype{
    pub fn copy_mutate(&self, defs: &ProgDefaults) -> Genotype{
        mutate(&self, defs)
    }
}


pub fn mutate(genome: &Genotype, defs: &ProgDefaults) -> Genotype{
    let ps = (0.5, 0.25, 0.25);

    let r = rand::thread_rng().gen::<f32>();

    if r < ps.0{
        micro_mutate(genome, defs)
    }
    else if r < ps.0 + ps.1 {
        insert_mutate(genome, defs)
    }
    else {
        del_mutate(genome, defs)
    }
}

pub fn micro_mutate(genome: &Genotype, defs: &ProgDefaults) -> Genotype {
    let len = genome.len();
    if len == 0{
        return Genotype::new(defs)
    }

    let mut new = genome.copy();
    let mut rng = rand::thread_rng();
    let mut_point = rng.gen_range(0, len);

    let new_gene = match new.raw_seq[mut_point] {
        0..=N_OPS => rng.gen_range(0, N_OPS),
        _ => rng.gen_range(N_OPS, GENE_MAX),
    };

    new.show_mutable_geno()[mut_point] = new_gene;
    new
}


pub fn insert_mutate(genome: &Genotype, defs: &ProgDefaults) -> Genotype {
    let mut rng = rand::thread_rng();

    let nops = count_ops(&genome.raw_seq);

    if nops == 0 {
        return Genotype::new(defs)
    }

    let insert_before = rng.gen_range(0, nops);
    let instr_size = rng.gen_range(defs.min_il, defs.max_il);



    let mut new_geno = Vec::with_capacity(genome.len() + instr_size+1);
    let mut instr_count = 0;

    for gene in genome.show_geno().iter(){

        if *gene < N_OPS {instr_count += 1}

        if instr_count== insert_before {
            new_geno.push(rng.gen_range(0, N_OPS));  // OP
            for _ in 0..instr_size{
                new_geno.push(rng.gen_range(N_OPS, GENE_MAX))  // ARGs
            }
        }

        new_geno.push(*gene);
    }

    Genotype{raw_seq:new_geno}
}


pub fn del_mutate(genome: &Genotype, defs: &ProgDefaults) -> Genotype {
    let mut rng = rand::thread_rng();

    let nops = count_ops(&genome.raw_seq);

    if nops == 0 {
        return Genotype::new(defs)
    }

    let del_ind = rng.gen_range(0, nops);


    let mut new_geno = Vec::with_capacity(genome.len());
    let mut instr_count = 0;

    for gene in genome.show_geno().iter(){

        if *gene < N_OPS {instr_count += 1}

        if instr_count != del_ind {
            new_geno.push(*gene);
        }
    }

    new_geno.shrink_to_fit();
    Genotype{raw_seq:new_geno}
}


fn count_ops(genome: &[GeneType])-> usize{
    genome.iter().filter(|x| **x > N_OPS ).count()
}
