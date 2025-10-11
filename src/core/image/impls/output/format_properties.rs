use crate::{encoding::ColorType, Image, Result};

impl Image {
    pub fn color_type(&self) -> Result<ColorType> {
        let img = self.processed_image();
        self.resolve_color_type(&*img, self.format()).map(Into::into)
    }

    pub fn has_transparency(&self) -> Result<bool> {
        let color_type = self.color_type()?;
        Ok(color_type.has_alpha())
    }

    pub fn is_grayscale(&self) -> Result<bool> {
        let color_type = self.color_type()?;
        Ok(color_type.is_grayscale())
    }

    pub fn bit_depth(&self) -> Result<u8> {
        let color_type = self.color_type()?;
        Ok(color_type.bit_depth())
    }

    pub fn channels(&self) -> Result<u8> {
        let color_type = self.color_type()?;
        Ok(color_type.channels())
    }
}
