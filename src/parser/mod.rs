use std::fs::File;

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
    let header = read_header::parse_header(&mut file)?;
    let variable_length_records = read_variable_length_records::parse_variable_length_records(&mut file, &header.number_vlrs)?;
    let point_records = read_point_records::parse_point_records(&mut file, &header.number_point_records)?;

    let las_file = LasFile {
        header,
        variable_length_records,
        point_records
    };

    Ok(las_file)
}