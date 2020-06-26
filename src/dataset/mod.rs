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
// use core::panicking::panic_fmt;


pub type YType = bool;
pub type XType = [f32; params::NFEAT];

pub const DEFAULT_X: XType = [0.0f32; params::NFEAT];
pub const DEFAULT_Y: YType = false;

#[derive(Copy, Clone)]
pub struct KDataSet{
    pub inds: [usize; params::NSAMP_TOT],
    pub x: [XType; params::NSAMP_TOT],
    pub y: [YType; params::NSAMP_TOT],
    pub tr_end: usize,
    pub cv_end: usize,
}

pub fn bare_gpmap(i: i32) -> f32{
    0.0
}
//
// pub fn test(){
//     (0usize..10usize).into_iter().enumerate()
// }

pub fn load_record(rec: &StringRecord, feats: &mut XType, from:usize, to:usize) {

    for (loop_i, rec_i) in (from..to).into_iter().enumerate() {
        match rec.get(rec_i) {
            Some(val) => {
                match val.parse::<f32>() {
                    Ok(val) => {
                        feats[loop_i] = val;
                    },
                    Err(e) => {
                        print!("Error reading something!! i's={},{} err is {:?}", rec_i, loop_i, e);
                        panic!("error getting inputs!, change code if dataset containt missing");
                    }
                }
            }
            None => panic!("No feature at {}, {}th iter of loop\nrec is {:?}", rec_i, loop_i, rec)
        }
    }
}


impl KDataSet{
    pub fn new_blank() -> KDataSet{
        KDataSet{
            inds: [0; params::NSAMP_TOT],
            x: [DEFAULT_X; params::NSAMP_TOT],
            y: [DEFAULT_Y; params::NSAMP_TOT],
            tr_end: 0,
            cv_end: 0,
        }
    }


    pub fn load_data(data_file: &str, ntr: usize, ncv:usize) -> KDataSet {
        println!("loading data from {}", data_file);

        assert!(ntr + ncv <= params::NSAMP_TOT, "Not enough samples!");

        let f = File::open(data_file).unwrap();
        let mut csv_rdr = ReaderBuilder::new()
            .delimiter(b'\t')
            .from_reader(f);

        let mut inds=  [0; params::NSAMP_TOT];
        let mut x = [DEFAULT_X; params::NSAMP_TOT];
        let mut y = [DEFAULT_Y; params::NSAMP_TOT];
        let mut tr_end = ntr;
        let mut cv_end = ntr+ncv;

        for (record_i, result) in csv_rdr.records().enumerate() {

            if let Ok(result) = result {

                load_record(&result, &mut x[record_i], 0, params::NFEAT);

                y[record_i] = if let Some(val) = result.get(LBL_COL){
                    match val {
                        "0" => false,
                        "1" => true,
                        _ => panic!("invalid lbl! {}. row, =  {}", val, record_i, )
                    }
                } else {panic!("lbl doesnt exist. row,=  {} {:?}", record_i, &result ) }

            }
            else {
                panic!("bad record! i={}, {:?}", record_i, &result);
            }
        }
        println!("data was loaded");
        KDataSet{ inds, x, y, tr_end, cv_end, }
    }


    pub fn is_empty(&self) -> bool {
        self.tr_end == 0
    }


    pub fn len(&self) -> usize{
        self.y.len()
    }

    pub fn shuffle(&mut self){
        let mut temp = *self;

        let mut ids: Vec<usize> = (0..NSAMP_TOT).collect();
        ids.shuffle(&mut rand::thread_rng());

        for (orig_i, new_i) in ids.into_iter().enumerate(){
            self.copy_row(&temp, new_i, orig_i);
            self.inds[orig_i] = new_i;
        }
    }


    pub fn shift_by(&mut self, n: usize){
        let mut temp = *self;

        for i in 0..self.len() {
            self.copy_row(&temp, (i+n)% self.len() , i);
        }
    }


    pub fn copy_row(&mut self, other: &KDataSet, to: usize, from: usize){
        self.y[to] = other.y[from];
        for i in 0..self.x[to].len(){
            self.x[to][i] = other.x[from][i];
        }
    }
}

//
//
// #[derive(Copy, Clone)]
// pub struct SampleX {
//     pub features: [f32; params::NFEAT],
//     pub id: usize,
//     pub gid: usize,
// }
//
// #[derive(Copy, Clone)]
// pub struct SampleY {
//     pub label: LabelType,
//     pub id: usize,
//     pub gid: usize,
// }
//
//
// pub struct DataXFull {
//     pub records: [SampleX; NSAMP_TOT],
// }
//
// pub struct DataYFull {
//     pub records: [SampleY; NSAMP_TOT],
// }
//
//
// pub trait DataX {
//     fn show_records(&self) -> &[SampleX];
// }
//
// pub trait DataY {
//     fn show_labels(&self) -> &[SampleY];
// }
//
// impl DataX for DataXFull{
//     fn show_records(&self)-> &[SampleX]{
//         self.records.as_ref()
//     }
// }
//
// impl DataY for DataYFull{
//     fn show_labels(&self)-> &[SampleY]{
//         self.records.as_ref()
//     }
// }
//
//
//
// impl SampleX {
//
//     fn new_blank()-> SampleX {
//         SampleX {
//             features: [0.0; NFEAT],
//             id:0,
//             gid:0,
//         }
//     }
//
//     fn update(&mut self, id: usize, gid: usize, feat_info: &StringRecord){
//         self.id = id;
//         self.gid = gid;
//         for (feature_i, next_value) in feat_info.iter().enumerate() {
//             match next_value.parse::<f32>() {
//                 Ok(entry) => {
//                     self.features[feature_i] = entry;
//                 },
//                 Err(e) => {
//                     print!("Error reading something!! i={} err is {:?}", feature_i, e);
//                     panic!("error getting inputs!, change code if dataset containt missing");
//                 }
//             }
//         }
//     }
// }
//
// impl SampleY{
//     fn new_blank() -> SampleY{
//         SampleY{
//             label:false,
//             id:0,
//             gid:0,
//         }
//     }
//
//     fn update(&mut self, lbl: LabelType, id: usize, gid: usize){
//         self.id = id;
//         self.gid = gid;
//         self.label = lbl;
//     }
// }
//
//
// // -> Box<DataSet>
// pub fn load_data(data_file: &str) -> (DataXFull, DataYFull) {
//     let mut rng = rand::thread_rng();
//
//     let f = File::open(data_file).unwrap();
//     let mut csv_rdr = ReaderBuilder::new()
//         .delimiter(b'\t')
//         .from_reader(f);
//
//
//     let mut ids: Vec<usize> = (0..NSAMP_TOT).collect();
//
//     if SHUFFLE { ids.shuffle(&mut rng); }
//
//     let mut X = [SampleX::new_blank(); NSAMP_TOT];
//     let mut Y = [SampleY::new_blank(); NSAMP_TOT];
//
//     for (record_i, result) in csv_rdr.records().enumerate() {
//         if let Ok(result) = result {
//             let id = ids[record_i];
//
//             X[id].update(id, record_i, &result);
//
//             let lbl = if Some(val) == result.get(LBL_COL){
//                 match val {
//                     "0" => false,
//                     "1" => true,
//                     _ => panic!("invalid lbl! {}. row, id = {}, {}", val, record_i, id)
//                 }
//             } else {panic!("lbl doesnt exist. row, id = {}, {} {}", val, record_i, id) };
//
//             Y[id].update(lbl, id, gid);
//         }
//         else {
//             panic!("bad record! i={}, {:?}", record_i, &result);
//         }
//     }
//     (DataXFull{records:X}, DataYFull{records:Y})
// }
//
//
//
//
//
//
//
//
//
// pub struct DataSetTR{
//     pub records: [SampleX; NSAMP_TRAIN],
// }
//
// pub struct DataSetCV{
//     pub records: [SampleX; NSAMP_CV],
// }
//
//
//
// pub struct DataLabelTR{
//     pub records: [SampleY; NSAMP_TRAIN],
// }
//
// pub struct DataLabelCV{
//     pub records: [SampleY; NSAMP_CV],
// }
//
//
//
//
//
// impl DataY for DataLabelTR{
//     fn show_labels(&self) -> &[SampleY] {
//         self.records.as_ref()
//     }
// }
//
// impl DataY for DataLabelCV{
//     fn show_labels(&self) -> &[SampleY] {
//         self.records.as_ref()
//     }
// }
// //
// // impl DataY for DataYFull {
// //     fn show_labels(&self) -> &[SampleY] {
// //         self.records.as_ref()
// //     }
// // }
//
//
//
// impl DataX for DataSetTR{
//     fn show_records(&self) -> &[SampleX]{
//         self.records.as_ref()
//     }
// }
//
// impl DataX for DataSetCV{
//     fn show_records(&self) -> &[SampleX]{
//         self.records.as_ref()
//     }
// }
//
// // impl DataX for DataXFull {
// //     fn show_records(&self) -> &[SampleX]{
// //         self.records.as_ref()
// //     }
// // }
//
// ////////////////////////////////////////////////////////
//
//
//
//
//
// // pub fn load_record(result: &StringRecord, feat_array: &mut DataFeats) {
// //     for (feature_i, next_value) in result.iter().enumerate() {
// //         match next_value.parse::<f32>() {
// //             Ok(entry) => {
// //                 feat_array.features[feature_i] = entry;
// //             },
// //             Err(e) => {
// //                 print!("Error reading something!! i={} err is {:?}", feature_i, e);
// //                 panic!("error getting inputs!, change code if dataset containt missing");
// //             }
// //         }
// //     }
// // }
//
//
// // // -> Box<DataSet>
// // pub fn load_data(data_file: &str) {
// //     let mut rng = rand::thread_rng();
// //
// //     let f = File::open(data_file).unwrap();
// //     let mut csv_rdr = ReaderBuilder::new()
// //         .delimiter(b'\t')
// //         .from_reader(f);
// //
// //
// //     let mut ids: Vec<usize> = (0..NSAMP_TOT).collect();
// //     ids.shuffle(&mut rng);
// //
// //
// //     let mut tr_ids: Vec<_> = ids.drain(..NSAMP_TRAIN).collect();
// //     tr_ids.sort_by(|a, b| b.cmp(a));
// //     let mut cv_ids: Vec<_> = ids.drain(..NSAMP_CV).collect();
// //     cv_ids.sort_by(|a, b| b.cmp(a));
// //
// //     let mut tr_i = tr_ids.pop();
// //     let mut cv_i = cv_ids.pop();
// //
// //     let mut tr_count = 0;
// //     let mut cv_count = 0;
// //
// //
// //     let mut data_tr = [SampleX::new_blank(); NSAMP_TRAIN];
// //     let mut data_cv = [SampleX::new_blank(); NSAMP_CV];
// //
// //
// //     for (record_i, result) in csv_rdr.records().enumerate() {
// //         if let Ok(result) = result {
// //             if let Some(i) = tr_i {
// //                 if i == record_i {
// //                     data_tr[tr_count].update(tr_count, record_i, &result);
// //                     tr_count += 1;
// //                 }
// //             }
// //             if let Some(i) = cv_i {
// //                 if i == record_i {
// //                     data_cv[i].update(i, record_i, &result);
// //                     cv_count += 1;
// //                 }
// //             }
// //             // match tr_i {
// //             //     None => {},
// //             //     Some(i) => {
// //             //
// //             //     }
// //             // }
// //
// //             // let mut class = None;
// //             // let mut features = [0.0f32; params::N_FEATURES as usize];
// //             // let mut feature_i = 0;
// //             //
// //             // for (j, next_entry) in result.iter().enumerate() {
// //             //     if j == params::LBL_IND{
// //             //         class = Some(match next_entry {
// //             //             "0" => false,
// //             //             "1" => true,
// //             //             _ => panic!("Invalid classification field!!")
// //             //         });
// //             //     }
// //             //     else if j >= params::FEAT_RNG.start && j < params::FEAT_RNG.end {
// //             //         match next_entry.parse::<f32>() {
// //             //             Ok(entry) => {
// //             //                 features[feature_i] = entry;
// //             //                 feature_i+= 1;
// //             //             },
// //             //             Err(e) => {
// //             //                 print!("Error reading something!! i={} j={} err is {:?}", feature_i, j, e);
// //             //                 panic!("error getting inputs!, change code if dataset containt missing");
// //             //             }
// //             //         }
// //             //     }
// //             // }
// //             // assert_eq!(params::N_FEATURES as usize, feature_i, "error in features");
// //             // match class {
// //             //     Some(class) => {
// //             //         records.push(DataRecord{features, class});
// //             //     },
// //             //     None => panic!("Error getting class!"),
// //             // }
// //         } else {
// //             panic!("bad record! i={}, {:?}", record_i, &result);
// //         }
// //     }
// // }
//     // Box::new(DataSet{records})
//
// pub fn new_arrary(data_file: &str)-> Array {
//     let mut rng = rand::thread_rng();
//
//     let f = File::open(data_file).unwrap();
//     let mut csv_rdr = ReaderBuilder::new()
//         .delimiter(b'\t')
//         .from_reader(f);
//
//     let mut all_data = Vec::with_capacity(params::NSAMP_TOT);
//     for (record_i, result) in csv_rdr.records().enumerate() {
//         if let Ok(result) = result {
//
//             let mut v = Vec::with_capacity(params::NFEAT);
//             for (j, next_entry) in result.iter().enumerate() {
//                 match next_entry.parse::<f32>() {
//                     Ok(entry) => {
//                         v.push(entry)
//                     },
//                     Err(e) => {
//                         print!("Error reading something!! j={} err is {:?}", j, e);
//                         panic!("error getting inputs!, change code if dataset containt missing");
//                     }
//                 }
//             }
//             all_data.push(v);
//         }
//     }
//     Array::from(&all_data)
// }