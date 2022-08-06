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

fn parse_string_four_bytes(file: &mut File) -> std::io::Result<String> {
    let mut bytes: [u8; 4] = [0; 4];
    file.read(&mut bytes)?;
    let result = match str::from_utf8(&bytes) {
        Ok(value) => value.to_string(),
        Err(e) => {
            panic!("Error parsing las file, {}", e);
        } 
    };
    Ok(result)
}

fn parse_string_eight_bytes(file: &mut File) -> std::io::Result<String> {
    let mut bytes: [u8; 8] = [0; 8];
    file.read(&mut bytes)?;
    let result = match str::from_utf8(&bytes) {
        Ok(value) => value.to_string(),
        Err(e) => {
            panic!("Error parsing las file, {}", e);
        } 
    };
    Ok(result)
}

fn parse_u8(file: &mut File) -> std::io::Result<u8> {
    let mut bytes: [u8; 1] = [0; 1];
    file.read(&mut bytes)?;
    Ok(u8::from_le_bytes(bytes))
}

fn parse_u16(file: &mut File) -> std::io::Result<u16> {
    let mut bytes: [u8; 2] = [0; 2];
    file.read(&mut bytes)?;
    Ok(u16::from_le_bytes(bytes))
}

fn parse_u32(file: &mut File) -> std::io::Result<u32> {
    let mut bytes: [u8; 4] = [0; 4];
    file.read(&mut bytes)?;
    Ok(u32::from_le_bytes(bytes))
}

fn parse_string_32_bytes(file: &mut File) -> std::io::Result<String> {
    let mut bytes: [u8; 32] = [0; 32];
    file.read(&mut bytes)?;
    let result = match str::from_utf8(&bytes) {
        Ok(value) => value.to_string(),
        Err(e) => {
            panic!("Error parsing las file, {}", e);
        } 
    };
    Ok(result)
}


fn parse_char(file: &mut File) -> std::io::Result<char> {
    let mut bytes: [u8; 1] = [0; 1];
    file.read(&mut bytes)?;
    Ok(bytes[0] as char)
}

fn parse_number_point_by_return(file: &mut File) -> std::io::Result<[u32; 5]> {
    let mut legacy_number_points_by_return: [u32; 5] = [0; 5];
    for slice_number in 0..5 {
        let mut slice_bytes: [u8; 4] = [0; 4];
        file.read(&mut slice_bytes)?;
        let number_points = u32::from_le_bytes(slice_bytes);
        legacy_number_points_by_return[slice_number] = number_points;
    }
    Ok(legacy_number_points_by_return)
}

fn parse_f64(file: &mut File) -> std::io::Result<f64> {
    let mut bytes: [u8; 8] = [0; 8];
    file.read(&mut bytes)?;
    Ok(f64::from_le_bytes(bytes))
}

pub fn parse_header(file: &mut File) -> std::io::Result<Header> {

    let file_signature = parse_string_four_bytes(file)?;
    let file_source_id = parse_u16(file)?;
    let global_encoding = parse_u16(file)?;

    let project_id_data_1 = parse_u32(file)?;
    let project_id_data_2 = parse_u16(file)?;
    let project_id_data_3 = parse_u16(file)?;
    let project_id_data_4 = parse_string_eight_bytes(file)?;

    let version_major = parse_u8(file)?;
    let version_minor = parse_u8(file)?;

    let system_identifier = parse_string_32_bytes(file)?;
    let generating_software = parse_string_32_bytes(file)?;
    let file_creation_day_of_year = parse_u16(file)?;
    let file_creation_year = parse_u16(file)?;
    let header_size = parse_u16(file)?;
    let offset_to_point_data = parse_u32(file)?;
    let number_vlrs = parse_u32(file)?;
    let point_data_record_format = parse_char(file)?;
    let point_data_record_length = parse_u16(file)?;
    let number_point_records = parse_u32(file)?;
    let number_point_by_return = parse_number_point_by_return(file)?;

    let x_scale_factor = parse_f64(file)?;
    let y_scale_factor = parse_f64(file)?;
    let z_scale_factor = parse_f64(file)?;

    let x_offset = parse_f64(file)?;
    let y_offset = parse_f64(file)?;
    let z_offset = parse_f64(file)?;

    let x_max = parse_f64(file)?;
    let x_min = parse_f64(file)?;
    let y_max = parse_f64(file)?;
    let y_min = parse_f64(file)?;
    let z_max = parse_f64(file)?;
    let z_min = parse_f64(file)?;

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