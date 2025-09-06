use {crate::Result, std::io::Read};

pub fn read_to_vec<R: Read>(mut reader: R) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    Ok(buffer)
}
