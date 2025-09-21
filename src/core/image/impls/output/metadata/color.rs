use {
    crate::{Image, ImageError, Result, Rgb},
    color_thief::{get_palette, ColorFormat},
};

impl Image {
    pub fn palette(&mut self) -> Result<Vec<Rgb>> {
        let img = self.processed_image()?;

        let palette = if img.color().has_alpha() {
            let rgba_img = img.to_rgba8();
            get_palette(rgba_img.as_raw(), ColorFormat::Rgba, 5, 5)
        } else {
            let rgb_img = img.to_rgb8();
            get_palette(rgb_img.as_raw(), ColorFormat::Rgb, 5, 5)
        };

        Ok(palette
            .map_err(ImageError::GetColors)?
            .into_iter()
            .map(|color| Rgb { r: color.r, g: color.g, b: color.b })
            .collect())
    }

    pub fn dominant_color(&mut self) -> Result<Rgb> {
        let palette = self.palette()?;
        palette.get(0).cloned().ok_or_else(|| ImageError::EmptyPalette)
    }
}
