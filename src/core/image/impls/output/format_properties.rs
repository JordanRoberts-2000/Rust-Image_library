use crate::{
    encoding::{
        AvifColorType, ColorType, JpegColorType, PngColorType, TiffColorType, WebpColorType,
    },
    EncodeFormat, Format, Image, Result,
};

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
        let decoded = self.decoded();
        let mut ct = decoded.color()?;

        if let Some(fmt) = self.encoding_format() {
            ct = match fmt {
                EncodeFormat::Png => self.resolve_color_type::<PngColorType>(&decoded)?.into(),
                EncodeFormat::Jpeg => self.resolve_color_type::<JpegColorType>(&decoded)?.into(),
                EncodeFormat::Gif => ColorType::Rgba8,
                EncodeFormat::Webp => self.resolve_color_type::<WebpColorType>(&decoded)?.into(),
                EncodeFormat::Tiff => self.resolve_color_type::<TiffColorType>(&decoded)?.into(),
                EncodeFormat::Avif => self.resolve_color_type::<AvifColorType>(&decoded)?.into(),
            }
        }

        Ok(ct)
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
