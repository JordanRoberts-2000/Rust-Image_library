use crate::{
    constants::{DEFAULT_AVIF_QUALITY, DEFAULT_AVIF_SPEED},
    encoders::AvifEncoder,
};

impl AvifEncoder {
    pub fn new() -> Self {
        Self {
            alpha_quality: DEFAULT_AVIF_QUALITY,
            quality: DEFAULT_AVIF_QUALITY,
            speed: DEFAULT_AVIF_SPEED,
            color_type: None,
            strip_unused_transparency: false,
        }
    }
}
