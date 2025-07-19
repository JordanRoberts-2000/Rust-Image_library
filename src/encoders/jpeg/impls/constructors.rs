use crate::{constants::DEFAULT_JPEG_QUALITY, encoders::JpegEncoder};

impl JpegEncoder {
    pub fn new() -> Self {
        Self {
            color_type: None,
            progressive: false,
            quality: DEFAULT_JPEG_QUALITY,
        }
    }

    pub fn progressive() -> Self {
        Self {
            color_type: None,
            progressive: true,
            quality: DEFAULT_JPEG_QUALITY,
        }
    }
}
