use {
    crate::{
        pixels::{Rgb, Rgba},
        ErrorKind, Image, Result, Rgb as RbgColor, WithOrigin,
    },
    color_thief::{get_palette, ColorFormat},
};

impl Image {
    pub fn palette(&self) -> Result<Vec<RbgColor>> {
        let decoded = self.decoded();

        let palette = if decoded.color()?.has_alpha() {
            get_palette(&decoded.as_bytes::<Rgba<u8>>(), ColorFormat::Rgba, 5, 5)
        } else {
            get_palette(&decoded.as_bytes::<Rgb<u8>>(), ColorFormat::Rgb, 5, 5)
        };

        Ok(palette
            .map_err(ErrorKind::GetColors)
            .with_origin(self.origin())?
            .into_iter()
            .map(|color| RbgColor { r: color.r, g: color.g, b: color.b })
            .collect())
    }

    pub fn dominant_color(&self) -> Result<RbgColor> {
        let palette = self.palette()?;
        palette.get(0).cloned().ok_or_else(|| ErrorKind::EmptyPalette).with_origin(self.origin())
    }
}
