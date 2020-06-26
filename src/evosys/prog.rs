use super::*;



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

    fn new(defs: &ProgDefaults) -> VLGeno{
        let mut rng = rand::thread_rng();
        let len = rng.gen_range(defs.min_ins, defs.max_ins);
        let mut new_geno = Vec::with_capacity(len*defs.max_il);
        let mut max_act_reg = defs.eff_regs;

        for _ in 0..len {
            new_geno.push(rng.gen_range(0, gp_maps::N_OPS as i32)); // OP

            for _ in 0..rng.gen_range(defs.min_il, defs.max_il){
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
