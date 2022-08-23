use crate::parser::LasFile;

pub fn get_ground_info(las_file: &LasFile) {
    let mut num_ground_points = 0;
    for point in &las_file.point_records {
        if point.classification == 2 {
            num_ground_points += 1;
        }
    }
    println!("Total number of points: {}", las_file.header.number_point_records);
    println!("Total number of ground points: {}", num_ground_points);
    println!("Percentage of ground points relative to total: {}", num_ground_points as f32 / las_file.header.number_point_records as f32);
}