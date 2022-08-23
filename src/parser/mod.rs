use std::{
    io::{Seek},
    fs::File,
};

mod read_header;
mod read_variable_length_records;
mod read_point_records;

#[allow(dead_code)]
pub struct LasFile {
    pub header: read_header::Header,
    pub variable_length_records: read_variable_length_records::VariableLengthRecords,
    pub point_records: read_point_records::PointRecords
}

pub fn read_file(file_name: &String) -> std::io::Result<LasFile> {
    let mut file = File::open(file_name)?;
    println!("File length: {}", file.metadata().unwrap().len());
    let header = read_header::parse_header(&mut file)?;
    println!("Current stream position: {}", file.stream_position()?);
    println!("Header size: {}", header.header_size);
    let variable_length_records = read_variable_length_records::parse_variable_length_records(&mut file, &header.number_vlrs)?;

    println!("Number of points: {}", header.number_point_records);

    println!("Current stream position: {}", file.stream_position()?);
    println!("Points offset: {}", header.offset_to_point_data);
    println!("Points size: {}", header.point_data_record_length);
    println!("Point id: {}", header.point_data_record_format);

    let point_records = read_point_records::parse_point_records(&mut file, &header)?;

    let las_file = LasFile {
        header,
        variable_length_records,
        point_records
    };

    let num_bytes = 26_496_276;
    let num_records = 5_904_960;

    Ok(las_file)
}