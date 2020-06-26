use toml::Value;
use std::time::Instant;


pub fn from_string_vec<T: std::str::FromStr + std::fmt::Debug>(sv: &Vec<String>) -> Vec<T>{

    let mut v = Vec::with_capacity(sv.len());
    for item in sv.iter(){
        match item.parse::<T>() {
            Ok(val) => v.push(val),
            Err(e) => panic!("got an err!"),
        }
    }
    v

    // sv.iter().map(
    //     |x| x.parse::<T>().unwrap()
    // ).collect()
}


// pub fn extract_typed_vec<T>(name: &str, table: &Value) -> Vec<T>
//     where T: std::str::FromStr + std::fmt::Debug
// {
//
//     match table.get(name) {
//         Some(x) => match x {
//             Value::Array(x) => x.iter().map({
//                 |val| val_str(val).parse::<T>()
//             }).collect(),
//
//             x => vec![val_str(x)],
//         }
//         None => panic!("key not found!\n Err getting {} from \n{:?}", name, table)
//     }
// }

pub fn extract_typed_vec<T>(name: &str, table: &Value) -> Vec<T>
    where T: std::str::FromStr + std::fmt::Debug
{
    // let sv = extract_string_vec(name, table);
    from_string_vec(&extract_string_vec(name, table))

    // let v = match table.get(name) {
    //     Some(x) => match x {
    //         Value::Array(x) => x.iter().map({
    //             |val| val_str(val)
    //         }).collect(),
    //
    //         x => vec![val_str(x)],
    //     }
    //     None => panic!("key not found!\n Err getting {} from \n{:?}", name, table)
    // }


}


pub fn extract_string_vec(name: &str, table: &Value) -> Vec<String> {

    match table.get(name) {
        Some(x) => match x {
            Value::Array(x) => x.iter().map({
                |val| val_str(val)
            }).collect(),

            x => vec![val_str(x)],
        }
        None => panic!("key not found!\n Err getting {} from \n{:?}", name, table)
    }
}

pub fn val_str(v: &Value) -> String{
    match v {
        Value::String(x) => x.clone(),
        Value::Integer(x) => format!("{:?}", x),
        Value::Float(x) => format!("{:?}", x),
        Value::Boolean(x) => format!("{:?}", x),
        _ => panic!("cant convert {:?} to string!", v)
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

// pub fn extract_val<T: std::str::FromStr + std::fmt::Debug>(val: &str, args: &toml::Value) -> T{
//
//     match args.get(val) {
//         Some(x) => {
//             // x.parse::<T>().unwrap()
//             match x.parse::<T>(){
//                 Ok(x) => x,
//                 Err(e) => {panic!("err parsing {}, \n{:?}", x, 0.0);}
//             }
//         }
//         None => {
//             panic!("pop size not found!!")
//         }
//     }
// }

pub fn extract_val_vec(name: &str, table: &Value) -> Vec<Value> {
    match table.get(name) {
        Some(x) => match x {
            Value::Array(x) => x.clone(),
            _ => panic!("bad type! expected array for {} found \n{:?}", name, x)
        }
        None => panic!("key not found!\n Err getting {} from \n{:?}", name, table)
    }
}

