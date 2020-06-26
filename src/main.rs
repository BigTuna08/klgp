
extern crate klgp;
extern crate rand;
extern crate csv;
extern crate rustlearn;
extern crate toml;

use klgp::evosys::{evohi, EA};

use klgp::dataset::{KDataSet, params};

use klgp::evosys::helpers::extract_string_vec;

use std::collections::HashMap;
use toml::{Value};

fn t(){
    let value = "foo = [1,2,3,4]".parse::<Value>().unwrap();

    let info = match std::fs::read_to_string("../../rust/klgp/config.toml") {
        Ok(info) => info,
        Err(e) => panic!("Error reading config file\n {:?}", e),
    };

    let config = toml::from_str::<toml::Value >(&info).unwrap();
    //println!("Hello, world!, {:?}, {:?}, {:?}\n{:?}", value, value["foo"], value["foo"][0], &config);

    for k in config.as_table().unwrap().keys(){
        println!("key = {}", k);
    }

    //println!("\n\n{:?}", &config["all"]);
    println!("\n\n{:?}", &config);
    println!("\n\n{:?}", &config["all"]);
}

fn test_run(){
    // let info = ;

    let info = match std::fs::read_to_string("config.toml") {
        Ok(info) => info,
        Err(e) => panic!("Error reading config file\n {:?}", e),
    };

    let config = toml::from_str::<toml::Value >(&info).unwrap();


    // let mut config = HashMap::new();
    // config.insert(String::from("pop_size"), String::from("1024"));
    // config.insert(String::from("min_fit"), String::from("0.005"));
    // config.insert(String::from("init_pop_size"), String::from("10000"));

    let mut ea = klgp::evosys::BasicEA::new(config);
    ea.run();
}
// let pop_size = extract_val::<usize>("pop_size", args);//args.get("pop_size");
// let min_fit = extract_val::<f32>("min_fit", args);
// let init_pop_size = extract_val::<usize>("init_pop_size", args);




fn toml_test(){
    // let value = "foo = [1,2,3,4]".parse::<Value>().unwrap();

    let info = match std::fs::read_to_string("config-test.toml") {
        Ok(info) => info,
        Err(e) => panic!("Error reading config file\n {:?}", e),
    };

    let config = toml::from_str::<toml::Value >(&info).unwrap();
    // println!("Hello, world!, {:?}, {:?}, {:?}\n{:?}", value, value["foo"], value["foo"][0], &config);


    // println!("\n\n{:?}", &config);
    // println!("\n\n{:?}", &config["all"]);
    // println!("\n\n{:?}", &config["mode"]);
    // println!("\n\n{:?}", &config["map"]);
    // println!("\n\n{:?}", &config["map"]["kstuff"]);
    print_val("full table", &config);

    // for x in config.as_table().unwrap().keys(){
    //     print_val(x, &config[x])
        // match &config[x] {
        //     Table(x) => {
        //
        //     }
        //     i => {
        //         println!("{} = {}")
        //     }
        // }

    // }
}

fn print_val(name: &str, val: &Value){
    match val {
        Value::Table(x) => {
            println!("Table [{:?}]", name);
            for key in x.keys(){
                // println!("{:?}", key);
                print_val(key, &val[key])
            }
        }
        x => println!("{} = {:?}", name, val)
    }
    // println!("end {}", name);
}



fn toml_test2(){
    // let value = "foo = [1,2,3,4]".parse::<Value>().unwrap();

    let info = match std::fs::read_to_string("config.toml") {
        Ok(info) => info,
        Err(e) => panic!("Error reading config file\n {:?}", e),
    };

    let config = toml::from_str::<toml::Value >(&info).unwrap();

    println!("\nconfig\n{:?}\n", &config);

    let c = config; //&config.as_table().unwrap()["all"];

    let a = extract_string_vec("pop_size", &c);
    let b = extract_string_vec("min_fit", &c);
    let c = extract_string_vec("data_locs", &c);

    // let loc = match  { }

    println!("\n\n{:?}", &a);
    println!("\n\n{:?}", &b);
    println!("\n\n{:?}", &c);
    // println!("\n\n{:?}", &config["map"]["kstuff"]);
    // print_val("full table", &config);

}



fn main() {
    println!("Hello, world!");
    // evohi();
    // let mut ds = KDataSet::load_data("C:/Users/BigTuna/Dropbox/par_lgp/penndata/oa.csv",
    // params::NSAMP_TRAIN,
    // params::NSAMP_CV);
    //
    // ds.shuffle();
    toml_test2();
    test_run();

    KDataSet::load_data("C:/Users/BigTuna/Dropbox/par_lgp/penndata/oa.csv", 200, 80);
}
