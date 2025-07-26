use crate::{
    constants::{
        DEFAULT_AVIF_QUALITY, DEFAULT_AVIF_SPEED, DEFAULT_JPEG_QUALITY, DEFAULT_WEBP_QUALITY,
    },
    image::ImageConfig,
    CompressionType,
};

impl ImageConfig {
    pub fn resolve_jpeg_config(&self) -> (u8, bool) {
        if let Some(jpeg_cfg) = &self.jpeg {
            (jpeg_cfg.quality.clamp(1, 100), jpeg_cfg.progressive)
        } else {
            let quality =
                self.quality.map(|q| q.clamp(1, 100) as u8).unwrap_or(DEFAULT_JPEG_QUALITY);

            let progressive = matches!(self.compression, CompressionType::Lossless);

            (quality, progressive)
        }
    }

    pub fn resolve_webp_config(&self) -> (CompressionType, u8) {
        if let Some(webp_cfg) = &self.webp {
            let compression =
                if webp_cfg.lossless { CompressionType::Lossless } else { CompressionType::Lossy };
            (compression, webp_cfg.quality.clamp(1, 100) as u8)
        } else {
            match self.compression {
                CompressionType::Lossless => (CompressionType::Lossless, 100),
                CompressionType::Lossy => (
                    CompressionType::Lossy,
                    self.quality.map(|q| q.clamp(1, 100) as u8).unwrap_or(DEFAULT_WEBP_QUALITY),
                ),
            }
        }
    }

    pub fn resolve_avif_config(&self) -> (u8, u8, u8) {
        match &self.avif {
            Some(cfg) => {
                (cfg.quality.clamp(1, 100), cfg.speed.clamp(1, 10), cfg.alpha_quality.clamp(1, 100))
            }
            None => {
                let fallback =
                    self.quality.map(|q| q.clamp(1, 100) as u8).unwrap_or(DEFAULT_AVIF_QUALITY);
                (fallback, DEFAULT_AVIF_SPEED, fallback)
            }
        }
    }
}
