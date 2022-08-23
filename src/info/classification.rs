use std::collections::HashMap;

use crate::parser::LasFile;

pub fn get_classification_info(las_file: &LasFile) {
    let mut classification_counter: HashMap<&u8, u64> = HashMap::new();
    println!("Extracting classification info...");
    for point in &las_file.point_records {
        *classification_counter.entry(&point.classification).or_insert(0) += 1;
    }
    for classification_value in 0..19 {
        let count_points = classification_counter.get(&classification_value);
        let final_count_points = match count_points {
            Some(value) => value,
            None => &0
        };
        println!("Classification value {}, Number of points: {}", classification_value, final_count_points);
    }
}