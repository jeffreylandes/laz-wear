use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::str;

fn read_file(file_name: &String) -> std::io::Result<()> {
    let mut file = File::open(file_name)?;
    read_header(&mut file)?;
    Ok(())
}

fn read_file_signature(file: &mut File) -> std::io::Result<String> {
    let file_signature: String;
    let mut file_signature_bytes: [u8; 4] = [0; 4];
    file.read(&mut file_signature_bytes)?;
    match str::from_utf8(&file_signature_bytes) {
        Ok(value) => {
            println!("Found file signature {}", value);
            file_signature = value.to_string();
        }
        Err(e) => {
            eprintln!("Encountered error parsing file signature {}", e);
            panic!("Error parsing las file");
        } 
    }
    Ok(file_signature)
}

fn read_file_source_id(file: &mut File) -> std::io::Result<()> {
    let mut source_id_bytes: [u8; 2] = [0; 2];
    file.read(&mut source_id_bytes)?;
    match str::from_utf8(&source_id_bytes) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Encountered error parsing source id {}", e);
            panic!("Error parsing las file");
        } 
    }
    Ok(())
}

fn read_file_global_encoding(file: &mut File) -> std::io::Result<()> {
    let mut global_encoding_bytes: [u8; 2] = [0; 2];
    file.read(&mut global_encoding_bytes)?;
    Ok(())
}

fn read_project_id_1(file: &mut File) -> std::io::Result<()> {
    let mut project_id_1_bytes: [u8; 4] = [0; 4];
    file.read(&mut project_id_1_bytes)?;
    Ok(())
}

fn read_project_id_2(file: &mut File) -> std::io::Result<()> {
    let mut project_id_2_bytes: [u8; 2] = [0; 2];
    file.read(&mut project_id_2_bytes)?;
    Ok(())
}

fn read_project_id_3(file: &mut File) -> std::io::Result<()> {
    let mut project_id_3_bytes: [u8; 2] = [0; 2];
    file.read(&mut project_id_3_bytes)?;
    Ok(())
}

fn read_project_id_4(file: &mut File) -> std::io::Result<String> {
    let project_id_4: String;
    let mut project_id_4_bytes: [u8; 8] = [0; 8];
    file.read(&mut project_id_4_bytes)?;
    match str::from_utf8(&project_id_4_bytes) {
        Ok(value) => project_id_4 = value.to_string(),
        Err(e) => {
            eprintln!("Encountered error parsing ProjectID - GUID Data 4 {}", e);
            panic!("Error parsing las file");
        } 
    }
    Ok(project_id_4)
}

fn read_version_major(file: &mut File) -> std::io::Result<String> {
    let version_major: String;
    let mut version_major_bytes: [u8; 1] = [0; 1];
    file.read(&mut version_major_bytes)?;
    match str::from_utf8(&version_major_bytes) {
        Ok(value) => version_major = value.to_string(),
        Err(e) => {
            eprintln!("Encountered error parsing Version Major {}", e);
            panic!("Error parsing las file");
        } 
    }
    Ok(version_major)
}

fn read_version_minor(file: &mut File) -> std::io::Result<String> {
    let version_minor: String;
    let mut version_minor_bytes: [u8; 1] = [0; 1];
    file.read(&mut version_minor_bytes)?;
    match str::from_utf8(&version_minor_bytes) {
        Ok(value) => version_minor = value.to_string(),
        Err(e) => {
            eprintln!("Encountered error parsing Version Major {}", e);
            panic!("Error parsing las file");
        }
    }
    Ok(version_minor)
}

fn read_system_identifier(file: &mut File) -> std::io::Result<String> {
    let system_identifier: String;
    let mut system_identifier_bytes: [u8; 32] = [0; 32];
    file.read(&mut system_identifier_bytes)?;
    match str::from_utf8(&system_identifier_bytes) {
        Ok(value) => system_identifier = value.to_string(),
        Err(e) => {
            eprintln!("Encountered error parsing System Identifier {}", e);
            panic!("Error parsing las file");
        } 
    }
    println!("System identifier: {}", system_identifier);
    Ok(system_identifier)
}

fn read_generating_software(file: &mut File) -> std::io::Result<String> {
    let generating_software: String;
    let mut generating_software_bytes: [u8; 32] = [0; 32];
    file.read(&mut generating_software_bytes)?;
    match str::from_utf8(&generating_software_bytes) {
        Ok(value) => generating_software = value.to_string(),
        Err(e) => {
            eprintln!("Encountered error parsing Generating Software {}", e);
            panic!("Error parsing las file");
        } 
    }
    println!("Generating softward: {}", generating_software);
    Ok(generating_software)
}

fn read_file_creation_day_of_year(file: &mut File) -> std::io::Result<u16> {
    let mut creation_day_of_year_bytes: [u8; 2] = [0; 2];
    file.read(&mut creation_day_of_year_bytes)?;
    let creation_day_of_year: u16 = u16::from_le_bytes(creation_day_of_year_bytes);
    Ok(creation_day_of_year)
}

fn read_file_creation_year(file: &mut File) -> std::io::Result<u16> {
    let mut creation_year_bytes: [u8; 2] = [0; 2];
    file.read(&mut creation_year_bytes)?;
    let creation_year = u16::from_le_bytes(creation_year_bytes);
    Ok(creation_year)
}

fn read_header_size(file: &mut File) -> std::io::Result<u16> {
    let mut header_size_bytes: [u8; 2] = [0; 2];
    file.read(&mut header_size_bytes)?;
    let header_size = u16::from_le_bytes(header_size_bytes);
    Ok(header_size)
}

fn read_offset_to_point_data(file: &mut File) -> std::io::Result<u32> {
    let mut offset_bytes: [u8; 4] = [0; 4];
    file.read(&mut offset_bytes)?;
    let offset: u32 = u32::from_le_bytes(offset_bytes);
    Ok(offset)
}

fn read_number_vlrs(file: &mut File) -> std::io::Result<u32> {
    let mut number_vlrs_bytes: [u8; 4] = [0; 4];
    file.read(&mut number_vlrs_bytes)?;
    let number_vlrs: u32 = u32::from_le_bytes(number_vlrs_bytes);
    Ok(number_vlrs)
}

fn read_point_data_record_format(file: &mut File) -> std::io::Result<char> {
    let mut point_data_format_bytes: [u8; 1] = [0; 1];
    file.read(&mut point_data_format_bytes)?;
    let point_data_record_format: char = point_data_format_bytes[0] as char;
    Ok(point_data_record_format)
}

fn read_point_data_record_length(file: &mut File) -> std::io::Result<u16> {
    let mut point_data_length_bytes: [u8; 2] = [0; 2];
    file.read(&mut point_data_length_bytes)?;
    let point_data_record_length: u16 = u16::from_le_bytes(point_data_length_bytes);
    Ok(point_data_record_length)
}

fn read_legacy_number_point_records(file: &mut File) -> std::io::Result<u32> {
    let mut number_point_records_bytes: [u8; 4] = [0; 4];
    file.read(&mut number_point_records_bytes)?;
    let number_point_records: u32 = u32::from_le_bytes(number_point_records_bytes);
    Ok(number_point_records)
}

fn read_legacy_number_point_by_return(file: &mut File) -> std::io::Result<()> {
    let mut number_of_point_by_return_bytes: [u8; 20] = [0; 20];
    file.read(&mut number_of_point_by_return_bytes)?;
    Ok(())
}

fn read_x_scale_factor(file: &mut File) -> std::io::Result<f64> {
    let mut x_scale_factor_bytes: [u8; 8] = [0; 8];
    file.read(&mut x_scale_factor_bytes)?;
    let x_scale_factor: f64 = f64::from_le_bytes(x_scale_factor_bytes);
    Ok(x_scale_factor)
}

fn read_y_scale_factor(file: &mut File) -> std::io::Result<f64> {
    let mut y_scale_factor_bytes: [u8; 8] = [0; 8];
    file.read(&mut y_scale_factor_bytes)?;
    let y_scale_factor: f64 = f64::from_le_bytes(y_scale_factor_bytes);
    Ok(y_scale_factor)
}

fn read_z_scale_factor(file: &mut File) -> std::io::Result<f64> {
    let mut z_scale_factor_bytes: [u8; 8] = [0; 8];
    file.read(&mut z_scale_factor_bytes)?;
    let z_scale_factor: f64 = f64::from_le_bytes(z_scale_factor_bytes);
    Ok(z_scale_factor)
}

fn read_x_offset(file: &mut File) -> std::io::Result<f64> {
    let mut x_offset_bytes: [u8; 8] = [0; 8];
    file.read(&mut x_offset_bytes)?;
    let x_offset: f64 = f64::from_le_bytes(x_offset_bytes);
    Ok(x_offset)
}

fn read_y_offset(file: &mut File) -> std::io::Result<f64> {
    let mut y_offset_bytes: [u8; 8] = [0; 8];
    file.read(&mut y_offset_bytes)?;
    let y_offset: f64 = f64::from_le_bytes(y_offset_bytes);
    Ok(y_offset)
}

fn read_z_offset(file: &mut File) -> std::io::Result<f64> {
    let mut z_offset_bytes: [u8; 8] = [0; 8];
    file.read(&mut z_offset_bytes)?;
    let z_offset: f64 = f64::from_le_bytes(z_offset_bytes);
    Ok(z_offset)
}

fn read_min_x(file: &mut File) -> std::io::Result<f64> {
    let mut min_x_bytes: [u8; 8] = [0; 8];
    file.read(&mut min_x_bytes)?;
    let min_x: f64 = f64::from_le_bytes(min_x_bytes);
    Ok(min_x)
}

fn read_max_x(file: &mut File) -> std::io::Result<f64> {
    let mut max_x_bytes: [u8; 8] = [0; 8];
    file.read(&mut max_x_bytes)?;
    let max_x: f64 = f64::from_le_bytes(max_x_bytes);
    Ok(max_x)
}

fn read_min_y(file: &mut File) -> std::io::Result<f64> {
    let mut min_y_bytes: [u8; 8] = [0; 8];
    file.read(&mut min_y_bytes)?;
    let min_y: f64 = f64::from_le_bytes(min_y_bytes);
    Ok(min_y)
}

fn read_max_y(file: &mut File) -> std::io::Result<f64> {
    let mut max_y_bytes: [u8; 8] = [0; 8];
    file.read(&mut max_y_bytes)?;
    let max_y: f64 = f64::from_le_bytes(max_y_bytes);
    Ok(max_y)
}

fn read_min_z(file: &mut File) -> std::io::Result<f64> {
    let mut min_z_bytes: [u8; 8] = [0; 8];
    file.read(&mut min_z_bytes)?;
    let min_z: f64 = f64::from_le_bytes(min_z_bytes);
    Ok(min_z)
}

fn read_max_z(file: &mut File) -> std::io::Result<f64> {
    let mut max_z_bytes: [u8; 8] = [0; 8];
    file.read(&mut max_z_bytes)?;
    let max_z: f64 = f64::from_le_bytes(max_z_bytes);
    Ok(max_z)
}

fn read_header(file: &mut File) -> std::io::Result<()> {
    read_file_signature(file)?;
    read_file_source_id(file)?;
    read_file_global_encoding(file)?;
    read_project_id_1(file)?;
    read_project_id_2(file)?;
    read_project_id_3(file)?;
    read_project_id_4(file)?;
    read_version_major(file)?;
    read_version_minor(file)?;
    read_system_identifier(file)?;
    read_generating_software(file)?;
    let creation_day_of_year = read_file_creation_day_of_year(file)?;
    let creation_year = read_file_creation_year(file)?;
    read_header_size(file)?;
    let offset_to_point_data = read_offset_to_point_data(file)?;
    let number_vlrs = read_number_vlrs(file)?;
    read_point_data_record_format(file)?;
    read_point_data_record_length(file)?;
    read_legacy_number_point_records(file)?;
    read_legacy_number_point_by_return(file)?;

    let x_scale_factor = read_x_scale_factor(file)?;
    let y_scale_factor = read_y_scale_factor(file)?;
    let z_scale_factor = read_z_scale_factor(file)?;

    let x_offset = read_x_offset(file)?;
    let y_offset = read_y_offset(file)?;
    let z_offset = read_z_offset(file)?;

    let x_min = read_min_x(file)?;
    let x_max = read_max_x(file)?;
    let y_min = read_min_y(file)?;
    let y_max = read_max_y(file)?;
    let z_min = read_min_z(file)?;
    let z_max = read_max_z(file)?;

    println!("Creation day of year: {}", creation_day_of_year);
    println!("Creation year: {}", creation_year);
    println!("Offset to point data: {}", offset_to_point_data);
    println!("Number of variable length records: {}", number_vlrs);

    println!("x scale factor: {}", x_scale_factor);
    println!("y scale factor: {}", y_scale_factor);
    println!("z scale factor: {}", z_scale_factor);

    println!("x offset: {}", x_offset);
    println!("y offset: {}", y_offset);
    println!("z offset: {}", z_offset);

    println!("x min: {}", x_min);
    println!("x max: {}", x_max);
    println!("y min: {}", y_min);
    println!("y max: {}", y_max);
    println!("z min: {}", z_min);
    println!("z max: {}", z_max);

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Expected usage: laz-wear file.las");
        return;
    }
    let file_name = &args[1];
    match read_file(file_name) {
        Ok(_) => println!("Successfully parsed {}", file_name),
        Err(e) => eprintln!("Encountered error {}", e),
    }
}
