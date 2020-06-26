use super::*;

impl Population for BasicPop{

    fn prog_defs(&self) -> &ProgDefaults{
        &self.prog_defs
    }

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


    fn next_new(&mut self) -> Genotype{
        self.select_i = (self.select_i + 1) % self.progs.len();
        if self.progs[self.select_i].fit > self.min_fit{
            mutate(&self.progs[self.select_i].geno, &self.prog_defs)
        }
        else{self.next_new()}
    }

    fn init(&mut self, devel_env: &DevelEnv, eval_env: &EvalEnv){
        for _ in 0..self.pop_size{
            let geno = Genotype::new(&self.prog_defs);
            let pheno = gpmap(&geno, devel_env);
            let fit = pfmap(&pheno, eval_env);
            self.progs.push(Program{geno, pheno, fit});
        }
        if self.init_size > self.pop_size{
            for _ in self.pop_size..self.init_size{
                let geno = Genotype::new(&self.prog_defs);
                let pheno = gpmap(&geno, devel_env);
                let fit = pfmap(&pheno, eval_env);
                self.try_add(Program{geno, pheno, fit});
            }
        }
    }


    fn new(args: KPopConfig) -> BasicPop{


        let pop_size = args.pop_defs.pop_size;
        let init_pop_size = args.pop_defs.init_pop_size;
        let min_fit = args.pop_defs.min_fit;

        // let mut pop_size = extract_val_vec("pop_size", args);//args.get("pop_size");
        // let pop_size = pop_size.pop().expect("pop_size not found!")
        //     .as_integer().expect("pop size must be int!") as usize;
        //
        // let min_fit = extract_val_vec("min_fit", args).pop().expect("min_fit not found")
        //     .as_float().expect("min fit must be float!") as f32;
        //
        // let init_pop_size = extract_val_vec("init_pop_size", args).pop().expect("init_pop_size not found")
        //     .as_integer().expect("init_pop_size must be int!") as usize;

        assert!(pop_size <= init_pop_size);

        let mut progs = Vec::with_capacity(pop_size);


        BasicPop{
            progs, min_fit,
            select_i:0,
            prog_defs: args.prog_defs,
            pop_size: pop_size,
            init_size: init_pop_size
        }


    }


}
