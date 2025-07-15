use crate::{blocking::Image, Result};

impl Image {
    pub fn to_bytes(&mut self) -> Result<Vec<u8>> {
        self.apply_transforms()?;

        let mut buffer = Vec::new();
        self.encode(&mut buffer, self.format)?;

        Ok(buffer)
    }
}
