use crate::{encoding::ColorType, EncodeFormat, Format, Image, Result};

impl Image {
    pub fn format(&self) -> Option<Format> {
        self.format
    }

    pub fn encoding_format(&self) -> Option<EncodeFormat> {
        self.config
            .encode_format
            .or_else(|| self.format.and_then(|f| EncodeFormat::try_from(f).ok()))
    }

    pub fn color_type(&self) -> Result<ColorType> {
        let decoded = self.processed_decode();
        let img = decoded.get_static()?;
        img.color().try_into()
    }

    pub fn encoding_color_type(&self) -> Result<Option<ColorType>> {
        todo!()
    }

    pub fn has_transparency(&self) -> Result<bool> {
        Ok(self.color_type()?.has_alpha())
    }

    pub fn is_grayscale(&self) -> Result<bool> {
        Ok(self.color_type()?.is_grayscale())
    }

    pub fn bit_depth(&self) -> Result<u8> {
        Ok(self.color_type()?.bit_depth())
    }

    pub fn channels(&self) -> Result<u8> {
        Ok(self.color_type()?.channels())
    }
}
