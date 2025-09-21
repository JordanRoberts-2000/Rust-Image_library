use crate::{
    constants::{
        DEFAULT_AVIF_QUALITY, DEFAULT_AVIF_SPEED, DEFAULT_JPEG_QUALITY, DEFAULT_WEBP_QUALITY,
    },
    image::ImageConfig,
    AvifConfig, CompressionType, JpegConfig, PngConfig, WebpConfig,
};

impl ImageConfig {
    pub fn jpeg(&self) -> JpegConfig {
        if let Some(ref jpeg_cfg) = self.jpeg {
            JpegConfig {
                quality: jpeg_cfg.quality.clamp(1, 100),
                progressive: jpeg_cfg.progressive,
            }
        } else {
            let quality =
                self.quality.map(|q| q.clamp(1, 100) as u8).unwrap_or(DEFAULT_JPEG_QUALITY);
            JpegConfig { quality, progressive: false }
        }
    }

    pub fn png(&self) -> PngConfig {
        if let Some(ref png_cfg) = self.png {
            PngConfig { compression_type: png_cfg.compression_type }
        } else {
            PngConfig::default()
        }
    }

    pub fn webp(&self) -> WebpConfig {
        if let Some(ref webp_cfg) = self.webp {
            WebpConfig { quality: webp_cfg.quality.clamp(1, 100), lossless: webp_cfg.lossless }
        } else {
            match self.compression {
                CompressionType::Lossless => WebpConfig { quality: 100, lossless: true },
                CompressionType::Lossy => WebpConfig {
                    quality: self
                        .quality
                        .map(|q| q.clamp(1, 100) as u8)
                        .unwrap_or(DEFAULT_WEBP_QUALITY),
                    lossless: false,
                },
            }
        }
    }

    pub fn avif(&self) -> AvifConfig {
        if let Some(ref cfg) = self.avif {
            AvifConfig { quality: cfg.quality.clamp(1, 100), speed: cfg.speed.clamp(1, 10) }
        } else {
            let q = self.quality.map(|v| v.clamp(1, 100) as u8).unwrap_or(DEFAULT_AVIF_QUALITY);
            AvifConfig { quality: q, speed: DEFAULT_AVIF_SPEED }
        }
    }
}
