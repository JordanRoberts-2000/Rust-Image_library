use crate::{constants::DEFAULT_JPEG_QUALITY, encoders::JpegEncoder};

use super::super::Unset;

impl JpegEncoder<Unset> {
    pub fn new() -> Self {
        Self {
            color_type: None,
            progressive: false,
            quality: DEFAULT_JPEG_QUALITY,
            input: Unset,
        }
    }

    pub fn progressive() -> Self {
        Self {
            color_type: None,
            progressive: true,
            quality: DEFAULT_JPEG_QUALITY,
            input: Unset,
        }
    }
}
