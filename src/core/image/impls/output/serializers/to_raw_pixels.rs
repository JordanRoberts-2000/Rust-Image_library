use crate::{Image, PixelFormat, Result};

impl Image {
    pub fn to_raw_pixels<F: PixelFormat>(&self) -> Result<Vec<F::Channel>> {
        let decoded = self.decoded();
        let cow = F::from_decoded(&decoded);
        Ok(cow.into_owned())
    }
}
