use std::env;

mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Expected usage: laz-wear file.las");
        return;
    }
    let file_name = &args[1];
    let las_file: parser::LasFile = match parser::read_file(file_name) {
        Ok(parsed_file) => parsed_file,
        Err(e) => panic!("Encountered error {}", e),
    };
    println!("Successfully parsed {} with {} points", file_name, las_file.header.number_point_records);
}
