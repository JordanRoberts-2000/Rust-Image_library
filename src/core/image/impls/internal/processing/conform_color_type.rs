use {
    crate::{image::enums::ImageData, ColorType, Image, Result},
    image::DynamicImage,
};

impl Image {
    pub(super) fn conform_color_type(&self) -> Result<()> {
        let mut data = self.data.borrow_mut();

        let img: &mut DynamicImage = match &mut *data {
            ImageData::RawPixels(di) => di,
            _ => unreachable!("ensure_decoded guarantees RawPixels"),
        };

        let Some(target) = self.config.target_color_type else {
            return Ok(());
        };

        if img.color() == target.into() {
            return Ok(());
        }

        let new_img = match target {
            ColorType::L8 => DynamicImage::ImageLuma8(img.to_luma8()),
            ColorType::La8 => DynamicImage::ImageLumaA8(img.to_luma_alpha8()),
            ColorType::Rgb8 => DynamicImage::ImageRgb8(img.to_rgb8()),
            ColorType::Rgba8 => DynamicImage::ImageRgba8(img.to_rgba8()),
            ColorType::L16 => DynamicImage::ImageLuma16(img.to_luma16()),
            ColorType::La16 => DynamicImage::ImageLumaA16(img.to_luma_alpha16()),
            ColorType::Rgb16 => DynamicImage::ImageRgb16(img.to_rgb16()),
            ColorType::Rgba16 => DynamicImage::ImageRgba16(img.to_rgba16()),
            ColorType::Rgb32F => DynamicImage::ImageRgb32F(img.to_rgb32f()),
            ColorType::Rgba32F => DynamicImage::ImageRgba32F(img.to_rgba32f()),
        };

        *img = new_img;
        Ok(())
    }
}
