use crate::{Image, Result};

impl Image {
    pub fn to_raw_pixels(&mut self) -> Result<Vec<u8>> {
        self.apply_transforms()?;

        let decoded = self.get_decoded()?;
        Ok(decoded.to_rgba8().into_raw())
    }
}
