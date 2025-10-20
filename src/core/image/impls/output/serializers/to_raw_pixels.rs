use crate::{Image, PixelFormat, Result};

impl Image {
    pub fn to_raw_pixels<F: PixelFormat>(&self) -> Result<Vec<F::Channel>> {
        let decoded = self.processed_decode();
        let img = decoded.get_static()?;
        Ok(F::extract_raw_pixels(&img))
    }
}
