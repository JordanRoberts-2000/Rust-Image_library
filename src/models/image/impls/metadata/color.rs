use crate::{Image, ImageError, Result};

use color_thief::{get_palette, ColorFormat};

#[derive(Debug)]
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Rgb {
    pub fn rgb(&self) -> String {
        format!("rgb({}, {}, {})", self.r, self.g, self.b)
    }

    pub fn hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

impl Image {
    pub fn dominant_color(&mut self) -> Result<Rgb> {
        self.apply_transforms()?;

        let rgb_pixels = self.get_decoded()?.to_rgb8().into_raw();
        let palette =
            get_palette(&rgb_pixels, ColorFormat::Rgb, 5, 1).map_err(ImageError::GetColors)?;

        let dominant_color = palette.get(0).ok_or_else(|| ImageError::EmptyPalette)?;

        Ok(Rgb {
            r: dominant_color.r,
            g: dominant_color.g,
            b: dominant_color.b,
        })
    }

    pub fn palette(&mut self) -> Result<Vec<Rgb>> {
        self.apply_transforms()?;

        let rgb_pixels = self.get_decoded()?.to_rgb8().into_raw();

        let palette = get_palette(&rgb_pixels, ColorFormat::Rgb, 5, 5)
            .map_err(ImageError::GetColors)?
            .into_iter()
            .map(|color| Rgb {
                r: color.r,
                g: color.g,
                b: color.b,
            })
            .collect();

        Ok(palette)
    }
}
