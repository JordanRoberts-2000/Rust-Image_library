use webp::{Encoder, PixelLayout};

use crate::{ImageError, Result};

pub fn encode_webp_data(
    rgba: &image::RgbaImage,
    width: u32,
    height: u32,
    lossless: bool,
    quality: f32,
    id: &str,
) -> Result<Vec<u8>> {
    let encoder = Encoder::new(rgba.as_raw(), PixelLayout::Rgba, width, height);
    let encoded =
        encoder
            .encode_simple(lossless, quality)
            .map_err(|err| ImageError::WebPEncoding {
                err,
                id: id.to_string(),
            })?;

    Ok(encoded.to_vec())
}
