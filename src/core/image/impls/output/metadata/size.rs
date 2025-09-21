use crate::{Image, Result};

impl Image {
    pub fn size(&mut self) -> Result<u64> {
        let mut buffer = Vec::new();
        self.encode(&mut buffer, self.format())?;
        Ok(buffer.len() as u64)
    }

    pub fn pixel_data_size(&mut self) -> Result<u64> {
        let img = self.processed_image()?;
        Ok(img.as_bytes().len() as u64)
    }
}
