use colored::Colorize;

mod classification;
mod ground; 

use crate::parser::LasFile;

pub fn extract_file_info(las_file: &LasFile, desired_info: &String) {
    match desired_info.as_str() {
        "classification" => classification::get_classification_info(las_file),
        "ground" => ground::get_ground_info(las_file),
        _ => panic!("{}: unknown info request: {}", "error".red(), desired_info)
    }

}