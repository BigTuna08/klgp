use super::*;





impl EA for BasicEA{

    fn run(&mut self) -> i32{
        while let Some(config) = self.run_config.next_config()  {
            println!("running {:?}", &config);
            self.run_next(config);
        }
        0
    }


    fn run_next(&mut self, config: PConfig)->i32{
        let n_evals = config.n_eval;
        let cv_interval = self.log_freq;

        let start_time = Instant::now();
        let mut last_time = Instant::now();

        // make this get #'s from config!
        let eval_env = EvalEnv::load_data(&config.loc, 100, 40);

        // simplify
        let mut pop = PopType::new(KPopConfig{ pop_defs: config, prog_defs: self.prog_config});
        pop.init(&eval_env, &eval_env);

        for eval_i in 0..n_evals{
            let newgeno = pop.next_new();

            let pheno = gpmap(&newgeno, &eval_env);
            let fit = pfmap(&pheno, &eval_env);
            pop.try_add(Program{geno:newgeno, pheno, fit});

            if eval_i % cv_interval == 0 && eval_i > 0 {

                println!("After {} evals", eval_i);
                time_update(cv_interval as i32, &last_time);
                self.cv(&pop, &eval_env);

                last_time = Instant::now();
            }
        }

        println!("Overall time");
        time_update(n_evals as i32, &start_time);

        0
    }


    fn new(args: Value) -> BasicEA{
        match args{
            Value::Table(t) => {
                let max_iter = match t.get("n_iters")
                                         .expect("Error reading n_iters from config") {
                    Value::Integer(i) => *i as usize,
                    x => panic!("Need n_iters to be int! \ngot {:?}", x)
                };

                let log_freq = match t.get("log_freq")
                    .expect("Error reading log_freq from config") {
                    Value::Integer(i) => *i as usize,
                    x => panic!("Need log_freq to be int! \ngot {:?}", x)
                };

                let out_folder = match t.get("out_folder")
                                              .expect("Error reading out_folder from config") {
                    Value::String(i) => i.clone(),
                    x => panic!("Need out_folder to be String! \ngot {:?}", x)
                };


                let prog_config = KProgDefaults{
                    min_il: 1,
                    max_il: 5,
                    min_ins: 1,
                    max_ins: 10,
                    eff_regs: 5
                };

                BasicEA{ run_config:EARunner::from_table(&Value::Table(t)), max_iter, current_iter:0, out_folder, prog_config, log_freq}
            },
            x => panic!("Need args to be table! \ngot {:?}", x)
        }
    }





    fn cv(&self, pop: &impl Population, eval_env: &EvalEnv) -> CVResult{
        let scores_tr: Vec<f32> = pop.progs().iter().map(
            |prog| prog.fit
        ).collect();

        let scores_cv: Vec<f32> = pop.progs().iter().map(
            |prog| gp_maps::score_prog(&prog.geno.show_geno(),
                                       eval_env,
                                       eval_env.tr_end,
                                       eval_env.cv_end)
        ).collect();

        let ave_len = pop.progs().iter().map(
            |prog| prog.pheno.complexity
        ).sum::<usize>() as f64 / pop.progs().len() as f64;

        println!("On training, max fit = {} , min is = {}",
                 my_max(scores_tr.as_ref()), my_min(scores_tr.as_ref()));
        println!("On cv, max fit = {} , min is = {}\n",
                 my_max(scores_cv.as_ref()), my_min(scores_cv.as_ref()));
        println!("Ave complexity is {:?}", ave_len);
        my_max(scores_cv.as_ref())
    }
}




// let locs = extract_typed_vec::<String>("data_locs", args);
// let min_fits = from_string_vec::<f32>(&extract_string_vec("min_fit", args));
//
// for loc in extract_typed_vec::<String>("data_locs", args){
//     for min_fit in extract_typed_vec::<f32>("min_fit", args){
//         for n_evals in extract_typed_vec::<usize>("n_evals", args){
//             for pop_size in extract_typed_vec::<usize>("pop_size", args) {
//                 for init_pop_size in extract_typed_vec::<usize>("init_pop_size", args){
//
//                 }
//             }
//         }
//     }
// }


// fn new(args: toml::Value) -> BasicEA{
//
//     // let data = //Box::new(
//     //     dataset::KDataSet::load_data("C:/Users/BigTuna/Dropbox/par_lgp/penndata/oa.csv",
//     //                                  data_params::NSAMP_TRAIN,
//     //                                  data_params::NSAMP_CV);
//     // // );
//     //
//     // let mode = args.as_table().expect("Args must be table!")["mode"]
//     //     .as_str().expect("mode must be str");
//     // if mode != "map" {panic!("only map supported for now!")}
//
//     // let c = args.to_owned().as_table().unwrap().to_owned()
//     let config = args.as_table().expect("Args must be table!");
//
//     // let pop = PopType::new(args, &data, &data);
//     BasicEA{config:config.to_owned() }
// }