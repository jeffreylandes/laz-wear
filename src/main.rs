use std::env;
use std::fs::File;

mod read_header;
mod read_variable_length_records;
mod read_point_records;

#[allow(dead_code)]
struct LasFile {
    header: read_header::Header,
    variable_length_records: read_variable_length_records::VariableLengthRecords
}

fn read_file(file_name: &String) -> std::io::Result<LasFile> {
    let mut file = File::open(file_name)?;
    let header = read_header::parse_header(&mut file)?;
    let variable_length_records = read_variable_length_records::parse_variable_length_records(&mut file, &header.number_vlrs)?;
    let point_records = read_point_records::parse_point_records(&mut file, &header.number_point_records)?;

    println!("Parsed {} points", point_records.len());

    let las_file = LasFile {
        header,
        variable_length_records
    };

    Ok(las_file)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Expected usage: laz-wear file.las");
        return;
    }
    let file_name = &args[1];
    let las_file: LasFile;
    match read_file(file_name) {
        Ok(parsed_file) => {
            println!("Successfully parsed {}", file_name);
            las_file = parsed_file;
        }
        Err(e) => panic!("Encountered error {}", e),
    }

    println!("File has {} points", las_file.header.number_point_records)
}
