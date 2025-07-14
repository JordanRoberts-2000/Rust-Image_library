use crate::{
    constants::{DEFAULT_AVIF_QUALITY, DEFAULT_AVIF_SPEED},
    image::ImageConfig,
};

pub fn resolve_avif_config(config: &ImageConfig) -> (f32, u8, f32) {
    match &config.avif {
        Some(cfg) => (
            cfg.quality.clamp(1, 100) as f32,
            cfg.speed.clamp(1, 10),
            cfg.alpha_quality.clamp(1, 100) as f32,
        ),
        None => {
            let fallback = config
                .quality
                .map(|q| q.clamp(1, 100) as f32)
                .unwrap_or(DEFAULT_AVIF_QUALITY as f32);
            (fallback, DEFAULT_AVIF_SPEED, fallback)
        }
    }
}
