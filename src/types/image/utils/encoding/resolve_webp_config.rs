use crate::{constants::DEFAULT_WEBP_QUALITY, image::ImageConfig, CompressionType};

pub fn resolve_webp_config(config: &ImageConfig) -> (bool, f32) {
    if let Some(cfg) = &config.webp {
        (cfg.lossless, cfg.quality.clamp(1, 100) as f32)
    } else {
        match config.compression {
            CompressionType::Lossless => (true, 0.0),
            CompressionType::Lossy => (
                false,
                config
                    .quality
                    .map(|q| q.clamp(1, 100) as f32)
                    .unwrap_or(DEFAULT_WEBP_QUALITY as f32),
            ),
        }
    }
}
