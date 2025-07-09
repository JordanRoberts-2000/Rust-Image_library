use std::io::Read;

use crate::{BlockingImage, ColorType, ImageError, IoError, Result};

impl BlockingImage {
    pub fn from_raw_reader(
        reader: impl Read,
        width: u32,
        height: u32,
        color_type: ColorType,
    ) -> Result<Self> {
        let expected_len = width as usize * height as usize * color_type.bytes_per_pixel();
        let mut pixels = Vec::with_capacity(expected_len);

        reader
            .take(expected_len as u64)
            .read_to_end(&mut pixels)
            .map_err(IoError::ReadStream)?;

        if pixels.len() != expected_len {
            return Err(ImageError::InvalidBuffer(color_type));
        }

        Self::from_raw_pixels(pixels, width, height, color_type)
    }
}
