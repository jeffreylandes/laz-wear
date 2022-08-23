use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;

use crate::parser::read_header::Header;

#[allow(dead_code)]
pub struct PointRecord {
    x: f32,
    y: f32,
    z: f32,
    intensity: u16,
    return_bits: u8,
    pub classification: u8,
    scan_angle_rank: char,
    user_data: u8,
    point_source_id: u16
}

pub type PointRecords = Vec<PointRecord>;

fn parse_long(file: &mut File) -> std::io::Result<f32> {
    let mut long_bytes: [u8; 4] = [0; 4];
    file.read(&mut long_bytes)?;
    Ok(f32::from_le_bytes(long_bytes))
}

fn parse_short(file: &mut File) -> std::io::Result<u16> {
    let mut intensity_bytes: [u8; 2] = [0; 2];
    file.read(&mut intensity_bytes)?;
    Ok(u16::from_le_bytes(intensity_bytes))
}

fn parse_byte(file: &mut File) -> std::io::Result<u8> {
    let mut bytes: [u8; 1] = [0; 1];
    file.read(&mut bytes)?;
    Ok(bytes[0])
}

fn parse_char(file: &mut File) -> std::io::Result<char> {
    let mut bytes: [u8; 1] = [0; 1];
    file.read(&mut bytes)?;
    Ok(bytes[0] as char)
}

fn parse_point_record(file: &mut File) -> std::io::Result<PointRecord> {
    let x = parse_long(file)?;
    let y = parse_long(file)?;
    let z = parse_long(file)?;
    let intensity = parse_short(file)?;
    let return_bits = parse_byte(file)?; // TODO: Parse actual bits
    let classification = parse_byte(file)?;
    let scan_angle_rank = parse_char(file)?;
    let user_data = parse_byte(file)?;
    let point_source_id = parse_short(file)?;

    let point_record = PointRecord {
        x,
        y,
        z,
        intensity,
        return_bits,
        classification,
        scan_angle_rank,
        user_data,
        point_source_id
    };

    Ok(point_record)
}

pub fn parse_point_records(file: &mut File, header: &Header) -> std::io::Result<PointRecords> {
    let mut point_records: PointRecords = Vec::new();
    let num_records = header.number_point_records;
    let point_record_size = header.point_data_record_length;

    println!("Parsing {} point records", header.number_point_records);

    for record_number in 0..num_records {
        let point_start = header.offset_to_point_data + (record_number * point_record_size as u32);
        file.seek(SeekFrom::Start(point_start as u64))?;
        let point_record = parse_point_record(file)?;
        point_records.push(point_record);
    }

    Ok(point_records)
}