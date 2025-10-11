use crate::{
    constants::{DEFAULT_AVIF_QUALITY, DEFAULT_AVIF_SPEED},
    encoding::{AvifEncoder, AvifSpeed, Quality},
};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct AvifConfig {
    pub quality: Quality,
    pub speed: AvifSpeed,
}

impl Default for AvifConfig {
    fn default() -> Self {
        Self { quality: DEFAULT_AVIF_QUALITY.into(), speed: DEFAULT_AVIF_SPEED.into() }
    }
}

impl From<AvifConfig> for AvifEncoder {
    fn from(config: AvifConfig) -> Self {
        Self { quality: config.quality, speed: config.speed }
    }
}
