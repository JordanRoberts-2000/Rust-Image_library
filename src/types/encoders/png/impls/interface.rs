use {
    crate::{encoders::PngEncoder, IoError, Result},
    std::io::{Read, Write},
};

impl PngEncoder {
    pub fn from_bytes(&self, bytes: &[u8]) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        self.encode(bytes, &mut buffer)?;
        Ok(buffer)
    }

    pub fn from_reader(&self, mut reader: impl Read) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).map_err(IoError::ReadStream)?;

        self.from_bytes(&bytes)
    }

    pub fn write_to(&self, bytes: &[u8], writer: impl Write) -> Result<()> {
        self.encode(bytes, writer)
    }
}
