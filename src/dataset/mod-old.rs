pub mod params;

use params::*;
use csv;
use csv::{ReaderBuilder, StringRecord};
//use params as global_params;
use rand;
use rand::Rng;
use std::fs::File;

use rand::seq::SliceRandom;

use rustlearn::array::dense::Array;






#[derive(Copy, Clone)]
pub struct SampleX {
    pub features: [f32; params::NFEAT],
    pub id: usize,
    pub gid: usize,
}

#[derive(Copy, Clone)]
pub struct SampleY {
    pub label: LabelType,
    pub id: usize,
    pub gid: usize,
}


pub struct DataXFull {
    pub records: [SampleX; NSAMP_TOT],
}

pub struct DataYFull {
    pub records: [SampleY; NSAMP_TOT],
}


pub trait DataX {
    fn show_records(&self) -> &[SampleX];
}

pub trait DataY {
    fn show_labels(&self) -> &[SampleY];
}

impl DataX for DataXFull{
    fn show_records(&self)-> &[SampleX]{
        self.records.as_ref()
    }
}

impl DataY for DataYFull{
    fn show_labels(&self)-> &[SampleY]{
        self.records.as_ref()
    }
}



impl SampleX {

    fn new_blank()-> SampleX {
        SampleX {
            features: [0.0; NFEAT],
            id:0,
            gid:0,
        }
    }

    fn update(&mut self, id: usize, gid: usize, feat_info: &StringRecord){
        self.id = id;
        self.gid = gid;
        for (feature_i, next_value) in feat_info.iter().enumerate() {
            match next_value.parse::<f32>() {
                Ok(entry) => {
                    self.features[feature_i] = entry;
                },
                Err(e) => {
                    print!("Error reading something!! i={} err is {:?}", feature_i, e);
                    panic!("error getting inputs!, change code if dataset containt missing");
                }
            }
        }
    }
}

impl SampleY{
    fn new_blank() -> SampleY{
        SampleY{
            label:false,
            id:0,
            gid:0,
        }
    }

    fn update(&mut self, lbl: LabelType, id: usize, gid: usize){
        self.id = id;
        self.gid = gid;
        self.label = lbl;
    }
}


// -> Box<DataSet>
pub fn load_data(data_file: &str) -> (DataXFull, DataYFull) {
    let mut rng = rand::thread_rng();

    let f = File::open(data_file).unwrap();
    let mut csv_rdr = ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(f);


    let mut ids: Vec<usize> = (0..NSAMP_TOT).collect();

    if SHUFFLE { ids.shuffle(&mut rng); }

    let mut X = [SampleX::new_blank(); NSAMP_TOT];
    let mut Y = [SampleY::new_blank(); NSAMP_TOT];

    for (record_i, result) in csv_rdr.records().enumerate() {
        if let Ok(result) = result {
            let id = ids[record_i];

            X[id].update(id, record_i, &result);

            let lbl = if Some(val) == result.get(LBL_COL){
                match val {
                    "0" => false,
                    "1" => true,
                    _ => panic!("invalid lbl! {}. row, id = {}, {}", val, record_i, id)
                }
            } else {panic!("lbl doesnt exist. row, id = {}, {}", val, record_i, id) }

            Y[id].update(lbl, id, gid);
        }
        else {
            panic!("bad record! i={}, {:?}", record_i, &result);
        }
    }
    (DataXFull{records:X}, DataYFull{records:Y})
}









pub struct DataSetTR{
    pub records: [SampleX; NSAMP_TRAIN],
}

pub struct DataSetCV{
    pub records: [SampleX; NSAMP_CV],
}



pub struct DataLabelTR{
    pub records: [SampleY; NSAMP_TRAIN],
}

pub struct DataLabelCV{
    pub records: [SampleY; NSAMP_CV],
}





impl DataY for DataLabelTR{
    fn show_labels(&self) -> &[SampleY] {
        self.records.as_ref()
    }
}

impl DataY for DataLabelCV{
    fn show_labels(&self) -> &[SampleY] {
        self.records.as_ref()
    }
}
//
// impl DataY for DataYFull {
//     fn show_labels(&self) -> &[SampleY] {
//         self.records.as_ref()
//     }
// }



impl DataX for DataSetTR{
    fn show_records(&self) -> &[SampleX]{
        self.records.as_ref()
    }
}

impl DataX for DataSetCV{
    fn show_records(&self) -> &[SampleX]{
        self.records.as_ref()
    }
}

// impl DataX for DataXFull {
//     fn show_records(&self) -> &[SampleX]{
//         self.records.as_ref()
//     }
// }

////////////////////////////////////////////////////////





// pub fn load_record(result: &StringRecord, feat_array: &mut DataFeats) {
//     for (feature_i, next_value) in result.iter().enumerate() {
//         match next_value.parse::<f32>() {
//             Ok(entry) => {
//                 feat_array.features[feature_i] = entry;
//             },
//             Err(e) => {
//                 print!("Error reading something!! i={} err is {:?}", feature_i, e);
//                 panic!("error getting inputs!, change code if dataset containt missing");
//             }
//         }
//     }
// }


// // -> Box<DataSet>
// pub fn load_data(data_file: &str) {
//     let mut rng = rand::thread_rng();
//
//     let f = File::open(data_file).unwrap();
//     let mut csv_rdr = ReaderBuilder::new()
//         .delimiter(b'\t')
//         .from_reader(f);
//
//
//     let mut ids: Vec<usize> = (0..NSAMP_TOT).collect();
//     ids.shuffle(&mut rng);
//
//
//     let mut tr_ids: Vec<_> = ids.drain(..NSAMP_TRAIN).collect();
//     tr_ids.sort_by(|a, b| b.cmp(a));
//     let mut cv_ids: Vec<_> = ids.drain(..NSAMP_CV).collect();
//     cv_ids.sort_by(|a, b| b.cmp(a));
//
//     let mut tr_i = tr_ids.pop();
//     let mut cv_i = cv_ids.pop();
//
//     let mut tr_count = 0;
//     let mut cv_count = 0;
//
//
//     let mut data_tr = [SampleX::new_blank(); NSAMP_TRAIN];
//     let mut data_cv = [SampleX::new_blank(); NSAMP_CV];
//
//
//     for (record_i, result) in csv_rdr.records().enumerate() {
//         if let Ok(result) = result {
//             if let Some(i) = tr_i {
//                 if i == record_i {
//                     data_tr[tr_count].update(tr_count, record_i, &result);
//                     tr_count += 1;
//                 }
//             }
//             if let Some(i) = cv_i {
//                 if i == record_i {
//                     data_cv[i].update(i, record_i, &result);
//                     cv_count += 1;
//                 }
//             }
//             // match tr_i {
//             //     None => {},
//             //     Some(i) => {
//             //
//             //     }
//             // }
//
//             // let mut class = None;
//             // let mut features = [0.0f32; params::N_FEATURES as usize];
//             // let mut feature_i = 0;
//             //
//             // for (j, next_entry) in result.iter().enumerate() {
//             //     if j == params::LBL_IND{
//             //         class = Some(match next_entry {
//             //             "0" => false,
//             //             "1" => true,
//             //             _ => panic!("Invalid classification field!!")
//             //         });
//             //     }
//             //     else if j >= params::FEAT_RNG.start && j < params::FEAT_RNG.end {
//             //         match next_entry.parse::<f32>() {
//             //             Ok(entry) => {
//             //                 features[feature_i] = entry;
//             //                 feature_i+= 1;
//             //             },
//             //             Err(e) => {
//             //                 print!("Error reading something!! i={} j={} err is {:?}", feature_i, j, e);
//             //                 panic!("error getting inputs!, change code if dataset containt missing");
//             //             }
//             //         }
//             //     }
//             // }
//             // assert_eq!(params::N_FEATURES as usize, feature_i, "error in features");
//             // match class {
//             //     Some(class) => {
//             //         records.push(DataRecord{features, class});
//             //     },
//             //     None => panic!("Error getting class!"),
//             // }
//         } else {
//             panic!("bad record! i={}, {:?}", record_i, &result);
//         }
//     }
// }
    // Box::new(DataSet{records})

pub fn new_arrary(data_file: &str)-> Array {
    let mut rng = rand::thread_rng();

    let f = File::open(data_file).unwrap();
    let mut csv_rdr = ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(f);

    let mut all_data = Vec::with_capacity(params::NSAMP_TOT);
    for (record_i, result) in csv_rdr.records().enumerate() {
        if let Ok(result) = result {

            let mut v = Vec::with_capacity(params::NFEAT);
            for (j, next_entry) in result.iter().enumerate() {
                match next_entry.parse::<f32>() {
                    Ok(entry) => {
                        v.push(entry)
                    },
                    Err(e) => {
                        print!("Error reading something!! j={} err is {:?}", j, e);
                        panic!("error getting inputs!, change code if dataset containt missing");
                    }
                }
            }
            all_data.push(v);
        }
    }
    Array::from(&all_data)
}