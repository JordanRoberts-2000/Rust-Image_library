use crate::{Image, ImageError, Result, Rgb};

use color_thief::{get_palette, ColorFormat};
use tokio::task::spawn_blocking;

impl Image {
    pub async fn dominant_color(&mut self) -> Result<Rgb> {
        let pixels = self.to_raw_pixels().await?;

        let palette = spawn_blocking(move || {
            get_palette(&pixels, ColorFormat::Rgb, 5, 1).map_err(ImageError::GetColors)
        })
        .await
        .map_err(ImageError::TaskJoinError)??;

        let dominant_color = palette.get(0).ok_or_else(|| ImageError::EmptyPalette)?;

        Ok(Rgb {
            r: dominant_color.r,
            g: dominant_color.g,
            b: dominant_color.b,
        })
    }

    pub async fn palette(&mut self) -> Result<Vec<Rgb>> {
        let pixels = self.to_raw_pixels().await?;

        let palette = spawn_blocking(move || {
            get_palette(&pixels, ColorFormat::Rgb, 5, 1).map_err(ImageError::GetColors)
        })
        .await
        .map_err(ImageError::TaskJoinError)??
        .into_iter()
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
