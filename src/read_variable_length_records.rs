use std::fs::File;
use std::io::prelude::*;
use std::str;

#[allow(dead_code)]
pub struct VariableLengthRecord {
    reserved: u16,
    user_id: String,
    record_id: u16,
    record_length_after_header: u16,
    description: String
}

fn parse_reserved(file: &mut File) -> std::io::Result<u16> {
    let mut reserved_bytes: [u8; 2] = [0; 2];
    file.read(&mut reserved_bytes)?;
    Ok(u16::from_le_bytes(reserved_bytes))
}

fn parse_user_id(file: &mut File) -> std::io::Result<String> {
    let user_id: String;
    let mut user_id_bytes: [u8; 16] = [0; 16];
    file.read(&mut user_id_bytes)?;
    match str::from_utf8(&user_id_bytes) {
        Ok(value) => {
            user_id = value.to_string();
        }
        Err(e) => {
            eprintln!("Encountered error parsing variable length record user id {}", e);
            panic!("Error parsing las file");
        } 
    }
    Ok(user_id)
}

fn parse_record_id(file: &mut File) -> std::io::Result<u16> {
    let mut record_id_bytes: [u8; 2] = [0; 2];
    file.read(&mut record_id_bytes)?;
    Ok(u16::from_le_bytes(record_id_bytes))
}

fn parse_record_length_after_header(file: &mut File) -> std::io::Result<u16> {
    let mut record_length_after_bytes_header: [u8; 2] = [0; 2];
    file.read(&mut record_length_after_bytes_header)?;
    Ok(u16::from_le_bytes(record_length_after_bytes_header))
}

fn parse_description(file: &mut File) -> std::io::Result<String> {
    let descriptions: String;
    let mut description_bytes: [u8; 32] = [0; 32];
    file.read(&mut description_bytes)?;
    match str::from_utf8(&description_bytes) {
        Ok(value) => {
            descriptions = value.to_string();
        }
        Err(e) => {
            eprintln!("Encountered error parsing variable length record user id {}", e);
            panic!("Error parsing las file");
        } 
    }
    Ok(descriptions)
}

fn parse_vlr(file: &mut File) -> std::io::Result<VariableLengthRecord> {
    let reserved = parse_reserved(file)?;
    let user_id = parse_user_id(file)?;
    let record_id = parse_record_id(file)?;
    let record_length_after_header = parse_record_length_after_header(file)?;
    let description = parse_description(file)?;
    
    let vlr = VariableLengthRecord {
        reserved,
        user_id,
        record_id,
        record_length_after_header,
        description
    };

    Ok(vlr)
}

pub fn parse_variable_length_records(file: &mut File, num_vlrs: &u32) -> std::io::Result<Vec<VariableLengthRecord>> {
    let mut vlrs: Vec<VariableLengthRecord> = Vec::new();
    println!("Parsing {} variable length records", num_vlrs);
    for _ in 0..*num_vlrs {
        let new_vlr = parse_vlr(file)?;
        vlrs.push(new_vlr)
    }
    Ok(vlrs)
}