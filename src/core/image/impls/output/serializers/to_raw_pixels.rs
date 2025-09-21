use crate::{blocking::Image, RawColorType, RawColorTypeF32, RawColorTypeU16, Result};

impl Image {
    pub fn to_raw_pixels(&mut self, color_type: RawColorType) -> Result<Vec<u8>> {
        let cow = self.process_image()?;
        let img = cow.as_ref();

        let raw = match color_type {
            RawColorType::Rgb8 => img.to_rgb8().into_raw(),
            RawColorType::Rgba8 => img.to_rgba8().into_raw(),
            RawColorType::L8 => img.to_luma8().into_raw(),
            RawColorType::La8 => img.to_luma_alpha8().into_raw(),
        };

        Ok(raw)
    }

    pub fn to_raw_pixels_u16(&mut self, color_type: RawColorTypeU16) -> Result<Vec<u16>> {
        let cow = self.process_image()?;
        let img = cow.as_ref();

        let raw = match color_type {
            RawColorTypeU16::Rgb16 => img.to_rgb16().into_raw(),
            RawColorTypeU16::Rgba16 => img.to_rgba16().into_raw(),
            RawColorTypeU16::L16 => img.to_luma16().into_raw(),
            RawColorTypeU16::La16 => img.to_luma_alpha16().into_raw(),
        };

        Ok(raw)
    }

    pub fn to_raw_pixels_f32(&mut self, color_type: RawColorTypeF32) -> Result<Vec<f32>> {
        let cow = self.process_image()?;
        let img = cow.as_ref();

        let raw = match color_type {
            RawColorTypeF32::Rgb32F => img.to_rgb32f().into_raw(),
            RawColorTypeF32::Rgba32F => img.to_rgba32f().into_raw(),
        };

        Ok(raw)
    }
}
