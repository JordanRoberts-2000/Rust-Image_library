use crate::{Image, Result};

impl Image {
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        self.encode(&mut buffer, self.encoding_format())?;

        Ok(buffer)
    }
}
