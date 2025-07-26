use {
    crate::{blocking::Image, traits::ToColorThiefFormat, ImageError, Result, Rgb},
    color_thief::get_palette,
};

impl Image {
    pub fn palette(&mut self) -> Result<Vec<Rgb>> {
        let img = self.process_image()?;

        let palette = get_palette(
            &img.as_ref().as_bytes(),
            img.as_ref().color().to_color_thief_format(),
            5,
            5,
        )
        .map_err(ImageError::GetColors)?
        .into_iter()
        .map(|color| Rgb { r: color.r, g: color.g, b: color.b })
        .collect();

        Ok(palette)
    }

    pub fn dominant_color(&mut self) -> Result<Rgb> {
        let palette = self.palette()?;
        palette.get(0).cloned().ok_or_else(|| ImageError::EmptyPalette)
    }
}
