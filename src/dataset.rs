use std::collections::{HashSet, HashMap};

use itertools::Itertools;

use log::{ info, warn, debug };
use ordered_float::OrderedFloat;
use crate::tree::split::Split;
use arrow::csv;
use arrow::util::pretty::print_batches;
use arrow_csv::reader::Format;
use std::fs::File;
use std::io::Seek;
use std::sync::Arc;
use arrow::array::{ Array, Float64Array, StringArray };
use arrow::datatypes::{ DataType, Field, Schema };
use arrow::record_batch::RecordBatch;

pub struct DataSet {
    data: Vec<Vec<i16>>,
    label: Vec<f64>,
    weight: Option<Vec<f64>>,
    init_splits: Vec<Split>,
    threshes: Vec<Vec<f64>>,
    str2ind: Vec<HashMap<String, i16>>,
}

impl DataSet {

    pub fn from_csv(
        path: String,
        label: String,
        weight: String,
        all_feats: Vec<String>,
        cat_feats: Vec<String>,
        exlude_feats: Vec<String>,
        max_bin: i16,
        min_weight: f64
    ) -> Self {
        let (batch, label, weight) = Self::parse_csv(
            path,
            label,
            weight,
            all_feats,
            cat_feats,
            exlude_feats
        );

        let (threshes, mut data): (Vec<Vec<f64>>, Vec<Vec<i16>>) = batch
            .schema()
            .all_fields()
            .into_iter()
            .filter(|f| f.data_type().is_numeric())
            .into_iter()
            .map(|field| {
                let x: Vec<f64> = batch
                    .column_by_name(field.name())
                    .unwrap()
                    .as_any()
                    .downcast_ref::<Float64Array>()
                    .unwrap()
                    .iter()
                    .map(|i| {
                        match i {
                            Some(i) if !f64::is_nan(i) => i,
                            _ => 0.0,
                        }
                    })
                    .collect();
                let threshes = DataSet::find_threshes(&x, &weight, max_bin);
                info!("col:{}, threshes:{:?}", field.name(), threshes);
                let bin = x
                    .iter()
                    .map(|i| {
                        match threshes.binary_search_by(|s| s.total_cmp(i)) {
                            Ok(idx) => (idx as i16) + 1,
                            Err(idx) => idx as i16,
                        }
                    })
                    .collect();
                (threshes, bin)
            })
            .unzip();

        let (str2ind, x_cat): (Vec<HashMap<String, i16>>, Vec<Vec<i16>>) = batch
            .schema()
            .all_fields()
            .into_iter()
            .filter(|f| f.data_type().is_numeric())
            .into_iter()
            .map(|field| {
                let x: Vec<String> = batch
                    .column_by_name(field.name())
                    .unwrap()
                    .as_any()
                    .downcast_ref::<StringArray>()
                    .unwrap()
                    .iter()
                    .map(|i| i.unwrap_or("").to_owned())
                    .collect();
                let str2ind = DataSet::str_2_index(&x, &weight, max_bin, min_weight);
                let bin = x
                    .iter()
                    .map(|i| { str2ind[i] })
                    .collect();
                (str2ind, bin)
            })
            .unzip();

        data.extend(x_cat);
        let mut init_splits: Vec<Split> = threshes
            .iter()
            .enumerate()
            .map(|(i, th)| { 
                Split::new_continuous(i as u16, th.len() as i16) 
            })
            .collect();
        let n_con = init_splits.len() as u16;
        let cat_splits: Vec<Split> = str2ind
            .iter()
            .enumerate()
            .map(|(i, index)| {
                let i = (i as u16) + n_con;
                let cats = index.values().into_iter().copied().collect();
                Split::new_categorical(i + n_con, cats)
            })
            .collect();
        init_splits.extend(cat_splits);
        DataSet {
            data,
            label,
            weight,
            init_splits,
            threshes,
            str2ind,
        }
    }


    fn parse_csv(
        path: String,
        label: String,
        weight: String,
        all_feats: Vec<String>,
        cat_feats: Vec<String>,
        exlude_feats: Vec<String>
    ) -> (RecordBatch, Vec<f64>, Option<Vec<f64>>) {
        let all_feats: HashSet<String> = all_feats.into_iter().collect::<HashSet<String>>();
        let exlude_feats: HashSet<String> = exlude_feats.into_iter().collect::<HashSet<String>>();
        let cat_feats: HashSet<String> = cat_feats.into_iter().collect::<HashSet<String>>();

        let mut file = File::open(path).unwrap();
        let format = Format::default().with_header(true);
        let (schema, _) = format.infer_schema(&mut file, Some(100)).unwrap();

        assert!(schema.field_with_name(&label).is_ok(), "label: {} not exists", label);
        assert!(
            schema.field_with_name(&label).unwrap().data_type().is_numeric(),
            "label: {} is not numberic",
            label
        );

        let raw_colnames: Vec<String> = schema
            .all_fields()
            .iter()
            .map(|&f| f.name().to_string())
            .filter(|col| *col != label)
            .collect::<Vec<String>>();
        let colnames: HashSet<String> = HashSet::from_iter(raw_colnames.clone());
        assert!(
            colnames.len() == schema.all_fields().len() - 1,
            "csv header have duplicate colnames: {:?}",
            raw_colnames
        );

        let all_feats = if all_feats.len() == 0 {
            info!("all_feats is empty, will use all colnames");
            colnames
        } else {
            all_feats
        };

        if !exlude_feats.is_subset(&all_feats) {
            warn!("some exlude_feats:{:?} not exists, all:{:?}", exlude_feats, all_feats);
        }

        let all_feats: HashSet<String> = all_feats
            .difference(&exlude_feats)
            .map(|s| s.to_string())
            .collect();
        info!("exlude_feats:{:?}, all feats:{:?}", exlude_feats, all_feats);

        if !cat_feats.is_subset(&all_feats) {
            warn!("some cat_feats:{:?} not exists, all:{:?}", cat_feats, all_feats);
        }

        let cat_feats: HashSet<String> = if cat_feats.len() == 0 {
            all_feats
                .iter()
                .map(|col| schema.field_with_name(col).unwrap())
                .filter(|&f| { *f.name() != label && !f.data_type().is_numeric() })
                .map(|f| f.name().to_owned())
                .collect::<HashSet<String>>()
        } else {
            all_feats
                .intersection(&cat_feats)
                .map(|s| s.to_string())
                .collect()
        };
        info!("cat feats: {:?}", cat_feats);

        info!("old schema:{}", schema);
        let fields: Vec<Field> = schema
            .all_fields()
            .into_iter()
            .map(|f| {
                let f = f.clone();
                if cat_feats.contains(f.name()) {
                    f.with_data_type(DataType::Utf8)
                } else if all_feats.contains(f.name()) {
                    f.with_data_type(DataType::Float64)
                } else if *f.name() == label {
                    f.with_data_type(DataType::Float64)
                } else {
                    f
                }
            })
            .collect();
        let schema = Schema::new(fields);
        info!("new schema:{}", schema);

        file.rewind().unwrap();
        let builder = csv::ReaderBuilder::new(Arc::new(schema.clone())).with_format(format);
        let mut csv = builder.build(file).unwrap();
        let batch = csv.next().unwrap().unwrap();

        let y = batch
            .column_by_name(&weight)
            .unwrap()
            .as_any()
            .downcast_ref::<Float64Array>()
            .unwrap()
            .iter()
            .map(|i| i.unwrap_or(0.0))
            .collect::<Vec<f64>>();

        let w = match schema.field_with_name(&weight) {
            Ok(w) if w.data_type().is_numeric() => {
                let w = batch
                    .column_by_name(&weight)
                    .unwrap()
                    .as_any()
                    .downcast_ref::<Float64Array>()
                    .unwrap();
                Some(
                    w
                        .iter()
                        .map(|i| i.unwrap_or(0.0))
                        .collect::<Vec<f64>>()
                )
            }
            _ => None,
        };

        let (fileds, cols): (Vec<Field>, Vec<Arc<dyn Array>>) = all_feats
            .iter()
            .map(|col| {
                let filed = schema.field_with_name(col).unwrap().clone();
                let col = batch.column_by_name(&col).unwrap().to_owned();
                (filed, col)
            })
            .unzip();
        let sub_batch = RecordBatch::try_new(Arc::new(Schema::new(fileds)), cols).unwrap();
        debug!("data:{:?}", print_batches(&[sub_batch.clone()]).unwrap());
        (sub_batch, y, w)
    }

    fn pre(&self, x: RecordBatch, i: usize, feat_idx: usize) -> i16 {
        
        if feat_idx < self.threshes.len() {
            let xi = x
                .column(feat_idx)
                .as_any()
                .downcast_ref::<Float64Array>()
                .unwrap()
                .value(i);

            match self.threshes[feat_idx].binary_search_by(|s| s.total_cmp(&xi)) {
                Ok(idx) => (idx as i16) + 1,
                Err(idx) => idx as i16,
            }
        } else {
            1
        }
        
        // 0
    }

    pub fn data(&self) -> &Vec<Vec<i16>> {
        &self.data
    }

    pub fn label(&self) -> &Vec<f64> {
        &self.label
    }

    pub fn weight(&self) -> &Option<Vec<f64>> {
        &self.weight
    }

    pub fn str_2_index(
        x: &Vec<String>,
        w: &Option<Vec<f64>>,
        max_bin: i16,
        min_weight: f64
    ) -> HashMap<String, i16> {
        let hist: Vec<(String, f64)> = {
            let mut hist: Vec<(String, f64)> = x
                .iter()
                .enumerate()
                .into_grouping_map_by(|&xw| xw.1.clone())
                .fold(0.0, |acc, _key, val| {
                    match w {
                        Some(w) => { acc + w[val.0] }
                        None => acc + 1.0,
                    }
                })
                .into_iter()
                .map(|x| (x.0, x.1))
                .collect();
            hist.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            hist
        };

        let str_index = hist
            .iter()
            .enumerate()
            .map(|(idx, (k, n))| {
                let idx = if idx >= (max_bin as usize) || *n <= min_weight {
                    max_bin
                } else {
                    idx as i16
                };
                (k.clone(), idx)
            })
            .collect();

        str_index
    }

    pub fn find_threshes(x: &Vec<f64>, w: &Option<Vec<f64>>, max_bin: i16) -> Vec<f64> {
        let hist: Vec<(f64, f64)> = {
            let mut hist: Vec<(f64, f64)> = x
                .iter()
                .map(|&x| OrderedFloat(x))
                .enumerate()
                .into_grouping_map_by(|&xw| xw.1.clone())
                .fold(0.0, |acc, _key, val| {
                    match w {
                        Some(w) => { acc + w[val.0] }
                        None => acc + 1.0,
                    }
                })
                .into_iter()
                .map(|x| (x.0.0, x.1))
                .collect();
            hist.sort_by(|a, b| a.partial_cmp(b).unwrap());
            hist
        };

        if hist.len() <= 1 {
            return Vec::new();
        }

        if hist.len() < (max_bin as usize) {
            let mid_thresh: Vec<f64> = hist
                .windows(2)
                .map(|kv| { (kv[0].0 + kv[1].0) / 2.0 })
                .collect();
            return mid_thresh;
        }

        let stride = {
            let wsum = hist
                .iter()
                .map(|xw| xw.1)
                .sum::<f64>();
            wsum / ((max_bin as f64) + 1.0)
        };

        let mut thresh: Vec<f64> = Vec::new();
        let mut ind = 1;
        let mut cur_cnt = hist[0].1;
        let mut target_cnt = stride;
        while ind < hist.len() {
            let prev_cnt = cur_cnt;
            cur_cnt += hist[ind].1;
            let prev_gap = f64::abs(prev_cnt - target_cnt);
            let cur_gap = f64::abs(cur_cnt - target_cnt);
            // If adding count of current value to currentCount
            // makes the gap between currentCount and targetCount smaller,
            // previous value is a split threshold.
            if prev_gap < cur_gap {
                thresh.push((hist[ind - 1].0 + hist[ind].0) / 2.0);
                target_cnt += stride;
            }
            ind += 1;
        }
        thresh
    }

    // fn reference(&self) -> Option<Self> {
    //     self.reference
    // }

    // fn from_matrix(raw: Vec<Vec<f64>>) -> Self {
    // }

    // pub fn from_csv(path: String, label: String, all_feat: Vec<String>, cat_feat: Vec<String>) {
    //     // let path = format!(
    //     //     "test/data/automobile-slim.csv",
    //     //     // env!("CARGO_MANIFEST_DIR")
    //     // );
    //     let mut file = File::open(path).unwrap();
    //     let format = Format::default().with_header(true);
    //     let (schema, _) = format.infer_schema(&mut file, Some(100)).unwrap();
    //     info!("schema:{}", schema);
    //     println!("{:?}", schema.metadata());
    //     assert!(schema.field_with_name(&label).is_ok(), "label: {} not exists", label);

    //     assert!(
    //         schema.field_with_name(&label).unwrap().data_type().is_numeric(),
    //         "label: {} is not numberic",
    //         label
    //     );

    //     // let field = schema.field_with_name(field_name).unwrap();
    //     // field.data_type().is_numeric();

    //     // schema.field_with_name(&label).unwrap().
    //     // std::collections::HashSet::from(all_feat).is_superset(other)
    //     // all_feat.

    //     // schema.iter().for
    //     // println!("{:}", schema);
    //     // println!("{:?}", schema.field(1));
    //     // println!("{:?}", schema.field_with_name("y"));
    //     // println!("{:?}", schema.);

    //     //file.rewind().unwrap();

    //     let builder = csv::ReaderBuilder::new(Arc::new(schema)).with_format(format);
    //     let mut csv = builder.build(file).unwrap();
    //     let batch = csv.next().unwrap().unwrap();
    //     let col = batch.column_by_name("y").unwrap();
    //     let t = col.as_any().downcast_ref::<Float64Array>().unwrap();
    //     // t.iter().for_each(|i| pri)
    //     // let v = t.into_data();
    //     // t.value_unchecked(index)
    //     // batch.column(2)
    //     println!("{:?}", batch.column_by_name("y"));
    // }

    // pub fn get_hist<K: Hash + Eq + Clone>(x: &Vec<K>, w: &Option<Vec<f64>>) -> Vec<(K, f64)> {
    //     let hist: Vec<(K, f64)> = x
    //         .iter()
    //         // .map(|&x| OrderedFloat(x))
    //         .enumerate()
    //         .into_grouping_map_by(|&xw| xw.1)
    //         .fold(0.0, |acc, _key, val| {
    //             match w {
    //                 Some(w) => { acc + w[val.0] }
    //                 None => acc + 1.0,
    //             }
    //         })
    //         .into_iter()
    //         .map(|(k, v)| (k.clone(), v))
    //         .collect();
    //     // hist.sort_by(|a, b| a.partial_cmp(b).unwrap());
    //     hist
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_threshes() {
        // let v: Vec<f64> = vec![];
        // assert_eq!(
        //     DataSet::find_threshes(&v, &None, i16::MAX),
        //     vec![f64::NEG_INFINITY, f64::INFINITY]
        // );

        // let v: Vec<f64> = vec![3.0, 2.0, 3.0, 4.0];
        // println!("v:{:?}, s:{:?}", &v, DataSet::find_threshes(&v, &None, i16::MAX));
        // assert_eq!(
        //     DataSet::find_threshes(&v, &None, i16::MAX),
        //     vec![f64::NEG_INFINITY, 2.5, 3.6, f64::INFINITY]
        // );

        let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
        let s = [2, 5, 7];
        println!("{:?}", s.binary_search(&0));
        println!("{:?}", s.binary_search(&2));
        println!("{:?}", s.binary_search(&4));
        println!("{:?}", s.binary_search(&5));
        println!("{:?}", s.binary_search(&8));
    }

    // use std::fs::File;
    // use std::io::Seek;
    // use std::sync::Arc;
    // use arrow::array::{Array, Float64Array, Int32Array, Int64Array};
    // use arrow::datatypes::DataType;
    // use arrow::record_batch::RecordBatch;
    // use arrow_csv::reader::Format;
    // use log::info;

    // // #[test]
    // pub fn from_csv(
    //     // path: String,
    //     label: String,
    //     all_feat: Vec<String>,
    //     cat_feat: Vec<String>,
    // ) {
    //     let path = format!(
    //         "test/data/automobile-slim.csv",
    //         // env!("CARGO_MANIFEST_DIR")
    //     );
    //     let mut file = File::open(path).unwrap();
    //     let format = Format::default().with_header(true);
    //     let (schema, _) = format.infer_schema(&mut file, Some(100)).unwrap();
    //     info!("schema:{}", schema);
    //     println!("{:?}", schema.metadata());
    //     // let s = schema.field_with_name(&label).is;
    //     assert!(
    //         schema.field_with_name(&label).is_ok(),
    //         "label: {} not exists", label
    //     );

    //     assert!(
    //         schema.field_with_name(&label).unwrap().data_type().is_numeric(),
    //         "label: {} is not numberic", label
    //     );

    //     let builder = csv::ReaderBuilder::new(Arc::new(schema)).with_format(format);
    //     let mut csv = builder.build(file).unwrap();
    //     let batch = csv.next().unwrap().unwrap();

    //     println!("{:?}", batch.column_by_name("y"));
    // }

    // use arrow::csv;
    // use arrow::util::pretty::print_batches;
    // use arrow_csv::reader::Format;
    // use log::info;
    // use std::fs::File;
    // use std::io::Seek;
    // use std::sync::Arc;

    /*
    tmp <- read_csv('test/data/automobile.csv')
    tmp |>
        select(
            price, normalized_losses, make, wheel_drive, height, engine_size, horsepower
        ) |>
        rename(y = price) |> 
        write_csv('test/data/automobile-slim.csv', na = "")
    */

    #[test]
    fn from_csv(
        // path: String,
        // label: String,
        // all_feat: Vec<String>,
        // cat_feat: Vec<String>,
    ) {
        let path = format!("{}/test/data/automobile-slim.csv", env!("CARGO_MANIFEST_DIR"));
        let label: String = "y".to_owned();
        let weight: String = "w".to_owned();
        let all_feats: HashSet<String> = vec!["make", "wheel_drive", "height", "horsepower"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<HashSet<String>>();
        let exlude_feats: HashSet<String> = vec!["horsepower"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<HashSet<String>>();
        let cat_feats: HashSet<String> = vec!["make", "wheel_drive"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<HashSet<String>>();

        let max_bin = i16::MAX;
        let min_weight = 1.0;

        let mut file = File::open(path).unwrap();
        let format = Format::default().with_header(true);
        let (schema, _) = format.infer_schema(&mut file, Some(100)).unwrap();

        assert!(schema.field_with_name(&label).is_ok(), "label: {} not exists", label);
        assert!(
            schema.field_with_name(&label).unwrap().data_type().is_numeric(),
            "label: {} is not numberic",
            label
        );

        // if schema.field_with_name(&weight).is_ok() && schema.field_with_name(&label).unwrap().data_type().is_numeric()
        // assert!(schema.field_with_name(&weight).is_ok(), "label: {} not exists", weight);
        // assert!(
        //     schema.field_with_name(&label).unwrap().data_type().is_numeric(),
        //     "label: {} is not numberic",
        //     label
        // );

        let raw_colnames: Vec<String> = schema
            .all_fields()
            .iter()
            .map(|&f| f.name().to_string())
            .filter(|col| *col != label)
            .collect::<Vec<String>>();
        let colnames: HashSet<String> = HashSet::from_iter(raw_colnames.clone());
        assert!(
            colnames.len() == schema.all_fields().len() - 1,
            "csv header have duplicate colnames: {:?}",
            raw_colnames
        );

        let all_feats = if all_feats.len() == 0 {
            info!("all_feats is empty, will use all colnames");
            colnames
        } else {
            all_feats
        };

        if !exlude_feats.is_subset(&all_feats) {
            warn!("some exlude_feats:{:?} not exists, all:{:?}", exlude_feats, all_feats);
        }

        let all_feats: HashSet<String> = all_feats
            .difference(&exlude_feats)
            .map(|s| s.to_string())
            .collect();
        info!("all feats: {:?}", all_feats);

        if !cat_feats.is_subset(&all_feats) {
            warn!("some cat_feats:{:?} not exists, all:{:?}", cat_feats, all_feats);
        }

        let cat_feats: HashSet<String> = if cat_feats.len() == 0 {
            schema
                .all_fields()
                .iter()
                .filter(|&&f| { *f.name() != label && !f.data_type().is_numeric() })
                .map(|f| f.name().to_owned())
                .collect::<HashSet<String>>()
        } else {
            all_feats
                .intersection(&cat_feats)
                .map(|s| s.to_string())
                .collect()
        };
        info!("cat feats: {:?}", cat_feats);

        println!("old schema");
        schema.fields.iter().for_each(|f| {
            println!("{}", f);
        });

        let mut fields: Vec<Field> = schema
            .all_fields()
            .into_iter()
            .map(|f| {
                let f = f.clone();
                if cat_feats.contains(f.name()) {
                    f.with_data_type(DataType::Utf8)
                } else if all_feats.contains(f.name()) {
                    f.with_data_type(DataType::Float64)
                } else if *f.name() == label {
                    f.with_data_type(DataType::Float64)
                } else {
                    f
                }
            })
            .collect();
        // fields.remove(0);
        let schema = Schema::new(fields);
        println!("new schema");
        // schema.fields.iter().for_each(|f| {println!("{}", f);});

        file.rewind().unwrap();

        let builder = csv::ReaderBuilder::new(Arc::new(schema.clone())).with_format(format);
        let mut csv = builder.build(file).unwrap();
        let batch = csv.next().unwrap().unwrap();

        // let n = batch.num_rows();
        // Vec::with_capacity(capacity)

        let w = match schema.field_with_name(&weight) {
            Ok(w) if w.data_type().is_numeric() => {
                let w = batch
                    .column_by_name(&weight)
                    .unwrap()
                    .as_any()
                    .downcast_ref::<Float64Array>()
                    .unwrap();
                Some(
                    w
                        .iter()
                        .map(|i| i.unwrap_or(0.0))
                        .collect::<Vec<f64>>()
                )
            }
            _ => None,
        };

        let (threshes, x_con): (Vec<Vec<f64>>, Vec<Vec<i16>>) = all_feats
            .difference(&cat_feats)
            .into_iter()
            .map(|col| {
                println!("{}", col);
                let x: Vec<f64> = batch
                    .column_by_name(col)
                    .unwrap()
                    .as_any()
                    .downcast_ref::<Float64Array>()
                    .unwrap()
                    .iter()
                    .map(|i| {
                        match i {
                            Some(i) if !f64::is_nan(i) => i,
                            _ => 0.0,
                        }
                    })
                    .collect();
                let threshes = DataSet::find_threshes(&x, &w, max_bin);
                println!("{:?}", threshes);
                let bin = x
                    .iter()
                    .map(|i| {
                        match threshes.binary_search_by(|s| s.total_cmp(i)) {
                            Ok(idx) => (idx as i16) + 1,
                            Err(idx) => idx as i16,
                        }
                        // idx as i16
                        // threshes.binary_search_by(|s| s.total_cmp(i)).unwrap() as i16
                    })
                    .collect();
                (threshes, bin)
            })
            .unzip();

        let (str2ind, x_cat): (Vec<HashMap<String, i16>>, Vec<Vec<i16>>) = cat_feats
            .iter()
            .map(|col| {
                let x: Vec<String> = batch
                    .column_by_name(col)
                    .unwrap()
                    .as_any()
                    .downcast_ref::<StringArray>()
                    .unwrap()
                    .iter()
                    .map(|i| i.unwrap_or("").to_owned())
                    .collect();
                let str2ind = DataSet::str_2_index(&x, &w, max_bin, min_weight);
                let bin = x
                    .iter()
                    .map(|i| { str2ind[i] })
                    .collect();
                (str2ind, bin)
            })
            .unzip();
        // batch.schema().

        // print_batches(&[batch]).unwrap();

        // batch.with_schema(schema)
        // let all_feats = ["y".to_owned(), "make".to_owned()];

        // let c = batch.columns(0);

        let (fileds, arr): (Vec<Field>, Vec<Arc<dyn Array>>) = all_feats
            .iter()
            .map(|col| {
                let filed = schema.field_with_name(col).unwrap().clone();
                let col = batch.column_by_name(&col).unwrap().to_owned();
                (filed, col)
            })
            .unzip();
        let sub_batch = RecordBatch::try_new(Arc::new(Schema::new(fileds)), arr).unwrap();
        print_batches(&[sub_batch]).unwrap();

        // let new_fields: Vec<Field> = new_fields.into_iter().map(|f| {
        //     f.with_data_type(DataType::Utf8)
        // }).collect();
        // schema.

        // let col: &Arc<dyn Array> = batch.column_by_name("y").unwrap();
        // col.as_any().downcast_ref::<Int32Array>().
        // col.
        // let tmp = [batch].get_mut(1).unwrap();
        // tmp.
        // println!("{:?}", batch.column_by_name("y"));

        // new_schema.all_fields().into_iter().for_each(|f| {
        //     f.with_data_type(DataType::Utf8)
        // })
        // new_fields.iter_mut().for_each(|f| {

        // }

        // new_schema.fields.iter().for_each(|f| {
        //     f.
        // f.with_data_type(DataType::Utf8)
        // f.as_ref().with_data_type(DataType::Utf8);
        // f.
        // f.with_data_type(DataType::Utf8);
        // f.as_ref().with_data_type(DataType::Utf8);
        // f.with_data_type(DataType::Utf8);
        // });
        // let new_schema = all_feats.iter().map(|col| -> arrow::datatypes::Field {
        //     let mut f = schema.field_with_name(col).unwrap().clone();
        //     if cat_feats.contains(f.name()) {
        //         f.with_data_type(DataType::Utf8)
        //     } else {
        //         f.with_data_type(DataType::Float64)
        //     };
        //     f
        // });

        //     let mut f = f.clone();
        //     (f).with_data_type(DataType::Utf8);
        //     // f
        //let mut s = schema.field_with_name(&label).unwrap().clone();
        // let mut s = schema.field_with_name(&label).unwrap();
        // schema.field_with_name(&label).unwrap().with_data_type(DataType::Utf8);

        // println!("{:?}", schema.all_fields());

        // schema.all_fields().iter().filter()
        // all_feats.intersection(&cat_feats)
        //     .map(|s| s.to_string())
        // .into_iter()
        //     .collect()
        // schema.all_fields().iter().filter(|&&f| {
        //     (*f.name() != label) && (!f.data_type().is_numeric())
        // })
        //     .map(|f| f.name().to_owned())
        //     .into_iter()
        //     .collect::<HashSet<String>>()
        // let cat_feats: HashSet<String> = all_feats.intersection(&cat_feats)
        //     .map(|s| s.to_string())
        //     .collect();

        // schema.
        // let new_schema = schema.all_fields().for_each(|f| {
        //     let mut f = f.clone();
        //     (f).with_data_type(DataType::Utf8);
        //     // f
        // });

        // schema.all_fields().iter_mut().for_each(|f| {
        //     f.with_data_type(DataType::Utf8)
        // });

        // println!("{:?}", schema.all_fields());
        // cat_feats.intersection(&all_feats).for_each(|col| {
        //     let mut s = schema.field_with_name(&col).unwrap().clone();
        //     // s.with_data_type(data_type)
        //     s.with_data_type(DataType::Utf8);
        //     s
        //     // schema.field_with_name(&col).unwrap().with_data_type(DataType::Utf8);
        // });

        // let cat_feats: HashSet<String> = all_feats.intersection(&cat_feats)
        //     .map(|s| s.to_string())
        //     .collect();

        // let cs1 = schema.field_with_name("make").unwrap();
        // let cs2 = schema.field_with_name("height").unwrap();
        // println!("cs1:{}, cs2:{}", cs1, cs2);

        //let mut s = schema.field_with_name(&label).unwrap().clone();
        // let mut s = schema.field_with_name(&label).unwrap();
        // schema.field_with_name(&label).unwrap().with_data_type(DataType::Utf8);

        // all_feats

        // let cat_feats

        // let r: HashSet<String> = colnames.difference(&exlude_feats.into_iter()).collect();
        // let all_feats: HashSet<String> = if all_feats.len() == 0 {
        //     // raw_colnames
        //     //     .into_iter()
        //     //     .filter(|col| exlude_feats.contains(col))
        //     //     .collect()
        //     let r = colnames.difference(&exlude_feats.into_iter().collect());

        // } else {
        //     let all_feats_uniq = HashSet::from_iter(all_feats.clone());
        //     assert!(
        //         colnames.is_superset(&all_feats_uniq),
        //         "all_feat_uniq:{:?} not in colnames:{:?}", all_feats_uniq, colnames
        //     );
        //     all_feats_uniq
        //         .into_iter()
        //         .filter(|col| exlude_feats.contains(col))
        //         .collect()
        // };
        // let all_feats = all_feats.retain(|col| exlude_feats.contains(col));

        // all_feats_uniq.retain(|col| exlude_feats.contains(col));

        // let all_feat: Vec<String> = vec!["make".to_owned(), "wheel_drive".to_owned(), "height".to_owned()];
        // let exlude_feat = vec!["horsepower".to_owned()];

        // let colnames = rcolnames.iter()
        // .collect::<HashSet<String>>();

        // let colname_names = schema.map(|f| f.nam)
        // info!("schema:{}", schema);
        // println!("{:?}", schema.fields);
        // let s = schema.field_with_name(&label).unwrap();
        // let s2 = s.clone().with_name("new");
        // schema/
        // HashSet::from(colname_names);
        // HashSet::from_iter(colname_names);
        // println!("all:{:?}", colname_names);
        // HashSet::from_iter(colname_names).len();
        // let viking_names = HashSet::from(["Einar".to_string(), "Olaf".to_string(), "Harald".to_string()]);
        // println!("all:{:?}", colname_names);
    }
}

// reference: Option<&Self>,
// all_feat_names: Vec<String>,
// categorical_feat_names: Vec<String>,
// constant_feat_names: Vec<String>,
