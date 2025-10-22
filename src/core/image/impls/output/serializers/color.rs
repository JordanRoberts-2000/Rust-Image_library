use {
    crate::{ErrorKind, Image, Result, Rgb, WithSrc},
    color_thief::{get_palette, ColorFormat},
};

impl Image {
    pub fn palette(&self) -> Result<Vec<Rgb>> {
        let decoded = self.processed_decode();
        let img = decoded.into_static()?;

        let palette = if img.color().has_alpha() {
            let rgba_img = img.to_rgba8();
            get_palette(rgba_img.as_raw(), ColorFormat::Rgba, 5, 5)
        } else {
            let rgb_img = img.to_rgb8();
            get_palette(rgb_img.as_raw(), ColorFormat::Rgb, 5, 5)
        };

        Ok(palette
            .map_err(ErrorKind::GetColors)
            .with_src(self.src())?
            .into_iter()
            .map(|color| Rgb { r: color.r, g: color.g, b: color.b })
            .collect())
    }

    pub fn dominant_color(&self) -> Result<Rgb> {
        let palette = self.palette()?;
        palette.get(0).cloned().ok_or_else(|| ErrorKind::EmptyPalette).with_src(self.src())
    }
}
