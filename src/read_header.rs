use std::fs::File;
use std::io::prelude::*;
use std::str;

// TODO: String --> char array
#[allow(dead_code)]
pub struct Header {
    file_signature: String,
    file_source_id: u16,
    global_encoding: u16,
    project_id_data_1: u32,
    project_id_data_2: u16,
    project_id_data_3: u16,
    project_id_data_4: String,
    version_major: u8,
    version_minor: u8,
    system_identifier: String,
    generating_software: String,
    file_creation_day_of_year: u16,
    file_creation_year: u16,
    header_size: u16,
    offset_to_point_data: u32,
    pub number_vlrs: u32,
    point_data_record_format: char,
    point_data_record_length: u16,
    pub number_point_records: u32,
    number_point_by_return: [u32; 5],
    x_scale_factor: f64,
    y_scale_factor: f64,
    z_scale_factor: f64,
    x_offset: f64,
    y_offset: f64,
    z_offset: f64,
    x_max: f64,
    x_min: f64,
    y_max: f64,
    y_min: f64,
    z_max: f64,
    z_min: f64
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

fn read_file_source_id(file: &mut File) -> std::io::Result<u16> {
    let mut source_id_bytes: [u8; 2] = [0; 2];
    file.read(&mut source_id_bytes)?;
    Ok(u16::from_le_bytes(source_id_bytes))
}

fn read_file_global_encoding(file: &mut File) -> std::io::Result<u16> {
    let mut global_encoding_bytes: [u8; 2] = [0; 2];
    file.read(&mut global_encoding_bytes)?;
    Ok(u16::from_le_bytes(global_encoding_bytes))
}

fn read_project_id_1(file: &mut File) -> std::io::Result<u32> {
    let mut project_id_1_bytes: [u8; 4] = [0; 4];
    file.read(&mut project_id_1_bytes)?;
    Ok(u32::from_le_bytes(project_id_1_bytes))
}

fn read_project_id_2(file: &mut File) -> std::io::Result<u16> {
    let mut project_id_2_bytes: [u8; 2] = [0; 2];
    file.read(&mut project_id_2_bytes)?;
    Ok(u16::from_le_bytes(project_id_2_bytes))
}

fn read_project_id_3(file: &mut File) -> std::io::Result<u16> {
    let mut project_id_3_bytes: [u8; 2] = [0; 2];
    file.read(&mut project_id_3_bytes)?;
    Ok(u16::from_le_bytes(project_id_3_bytes))
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

fn read_version_major(file: &mut File) -> std::io::Result<u8> {
    let mut version_major_bytes: [u8; 1] = [0; 1];
    file.read(&mut version_major_bytes)?;
    Ok(u8::from_le_bytes(version_major_bytes))
}

fn read_version_minor(file: &mut File) -> std::io::Result<u8> {
    let mut version_minor_bytes: [u8; 1] = [0; 1];
    file.read(&mut version_minor_bytes)?;
    Ok(u8::from_le_bytes(version_minor_bytes))
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
    println!("Generating software {}", generating_software);
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
    Ok(point_data_format_bytes[0] as char)
}

fn read_point_data_record_length(file: &mut File) -> std::io::Result<u16> {
    let mut point_data_length_bytes: [u8; 2] = [0; 2];
    file.read(&mut point_data_length_bytes)?;
    let point_data_record_length: u16 = u16::from_le_bytes(point_data_length_bytes);
    Ok(point_data_record_length)
}

fn read_number_point_records(file: &mut File) -> std::io::Result<u32> {
    let mut number_point_records_bytes: [u8; 4] = [0; 4];
    file.read(&mut number_point_records_bytes)?;
    let number_point_records: u32 = u32::from_le_bytes(number_point_records_bytes);
    Ok(number_point_records)
}

fn read_number_point_by_return(file: &mut File) -> std::io::Result<[u32; 5]> {
    let mut legacy_number_points_by_return: [u32; 5] = [0; 5];
    for slice_number in 0..5 {
        let mut slice_bytes: [u8; 4] = [0; 4];
        file.read(&mut slice_bytes)?;
        let number_points = u32::from_le_bytes(slice_bytes);
        legacy_number_points_by_return[slice_number] = number_points;
    }
    Ok(legacy_number_points_by_return)
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

pub fn parse_header(file: &mut File) -> std::io::Result<Header> {

    let file_signature = read_file_signature(file)?;
    let file_source_id = read_file_source_id(file)?;
    let global_encoding = read_file_global_encoding(file)?;

    let project_id_data_1 = read_project_id_1(file)?;
    let project_id_data_2 = read_project_id_2(file)?;
    let project_id_data_3 = read_project_id_3(file)?;
    let project_id_data_4 = read_project_id_4(file)?;

    let version_major = read_version_major(file)?;
    let version_minor = read_version_minor(file)?;

    let system_identifier = read_system_identifier(file)?;
    let generating_software = read_generating_software(file)?;
    let file_creation_day_of_year = read_file_creation_day_of_year(file)?;
    let file_creation_year = read_file_creation_year(file)?;
    let header_size = read_header_size(file)?;
    let offset_to_point_data = read_offset_to_point_data(file)?;
    let number_vlrs = read_number_vlrs(file)?;
    let point_data_record_format = read_point_data_record_format(file)?;
    let point_data_record_length = read_point_data_record_length(file)?;
    let number_point_records = read_number_point_records(file)?;
    let number_point_by_return = read_number_point_by_return(file)?;

    let x_scale_factor = read_x_scale_factor(file)?;
    let y_scale_factor = read_y_scale_factor(file)?;
    let z_scale_factor = read_z_scale_factor(file)?;

    let x_offset = read_x_offset(file)?;
    let y_offset = read_y_offset(file)?;
    let z_offset = read_z_offset(file)?;

    let x_max = read_max_x(file)?;
    let x_min = read_min_x(file)?;
    let y_max = read_max_y(file)?;
    let y_min = read_min_y(file)?;
    let z_max = read_max_z(file)?;
    let z_min = read_min_z(file)?;

    let header = Header {
        file_signature,
        file_source_id,
        global_encoding,
        project_id_data_1,
        project_id_data_2,
        project_id_data_3,
        project_id_data_4,
        version_major,
        version_minor,
        system_identifier,
        generating_software,
        file_creation_day_of_year,
        file_creation_year,
        header_size,
        offset_to_point_data,
        number_vlrs,
        point_data_record_format,
        point_data_record_length,
        number_point_records,
        number_point_by_return,
        x_scale_factor,
        y_scale_factor,
        z_scale_factor,
        x_offset,
        y_offset,
        z_offset,
        x_max,
        x_min,
        y_max,
        y_min,
        z_max,
        z_min,
    };

    Ok(header)
}