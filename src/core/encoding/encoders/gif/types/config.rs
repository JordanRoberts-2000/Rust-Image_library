use crate::{
    constants::DEFAULT_GIF_SPEED,
    encoding::{GifEncoder, GifRepeat, GifSpeed},
};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GifConfig {
    pub speed: GifSpeed,
    pub repeat: GifRepeat,
    pub animated: bool,
}

impl Default for GifConfig {
    fn default() -> Self {
        Self { speed: DEFAULT_GIF_SPEED.into(), repeat: GifRepeat::Infinite, animated: true }
    }
}

impl From<GifConfig> for GifEncoder {
    fn from(config: GifConfig) -> Self {
        Self { animated: config.animated, speed: config.speed, repeat: config.repeat }
    }
}
