use crate::{Image, PixelFormat, Result};

impl Image {
    pub fn to_raw_pixels<F: PixelFormat>(&self) -> Result<Vec<F::Channel>> {
        let img = self.processed_image();
        Ok(F::extract_raw_pixels(&img))
    }
}
