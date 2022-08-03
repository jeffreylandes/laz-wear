fn read_waveform_data_packet_record_start(file: &mut File) -> std::io::Result<u64> {
    let mut waveform_start_bytes: [u8; 8] = [0; 8];
    file.read(&mut waveform_start_bytes)?;
    let waveform_data_packet_record_start = u64::from_le_bytes(waveform_start_bytes);
    Ok(waveform_data_packet_record_start)
}

fn read_start_first_extended_vlr(file: &mut File) -> std::io::Result<u64> {
    let mut start_first_extended_vlr_bytes: [u8; 8] = [0; 8];
    file.read(&mut start_first_extended_vlr_bytes)?;
    let start_first_extended_vlr = u64::from_le_bytes(start_first_extended_vlr_bytes);
    Ok(start_first_extended_vlr)
}

fn read_number_extended_vlrs(file: &mut File) -> std::io::Result<u32> {
    let mut number_extended_vlrs_bytes: [u8; 4] = [0; 4];
    file.read(&mut number_extended_vlrs_bytes)?;
    let number_extended_vlrs = u32::from_le_bytes(number_extended_vlrs_bytes);
    Ok(number_extended_vlrs)
}

fn read_number_point_records(file: &mut File) -> std::io::Result<u64> {
    let mut number_point_records_bytes: [u8; 8] = [0; 8];
    file.read(&mut number_point_records_bytes)?;
    let number_point_records = u64::from_le_bytes(number_point_records_bytes);
    Ok(number_point_records)
}

fn read_number_points_by_return(file: &mut File) -> std::io::Result<[u64; 15]> {
    let mut number_points_by_return: [u64; 15] = [0; 15];
    for slice_number in 0..15 {
        let mut slice_bytes: [u8; 8] = [0; 8];
        file.read(&mut slice_bytes)?;
        let number_points = u64::from_le_bytes(slice_bytes);
        number_points_by_return[slice_number] = number_points;
    }
    Ok(number_points_by_return)
}