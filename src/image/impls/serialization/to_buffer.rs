use crate::{Image, Result};

impl Image {
    pub fn to_buffer(&mut self) -> Result<Vec<u8>> {
        self.apply_transforms()?;

        let mut buffer = Vec::new();
        self.encode(&mut buffer)?;

        Ok(buffer)
    }
}
