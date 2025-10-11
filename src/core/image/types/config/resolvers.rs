use crate::{
    constants::{DEFAULT_AVIF_QUALITY, DEFAULT_JPEG_QUALITY, DEFAULT_WEBP_QUALITY},
    encoding::{AvifConfig, CompressionType, JpegConfig, PngConfig, WebpConfig},
    image::ImageConfig,
};

impl ImageConfig {
    pub fn jpeg(&self) -> JpegConfig {
        self.jpeg.unwrap_or_else(|| JpegConfig {
            quality: self.quality.map(|q| q.into()).unwrap_or(DEFAULT_JPEG_QUALITY.into()),
            ..Default::default()
        })
    }

    pub fn png(&self) -> PngConfig {
        self.png.unwrap_or_else(PngConfig::default)
    }

    pub fn webp(&self) -> WebpConfig {
        self.webp.unwrap_or_else(|| match self.compression {
            CompressionType::Lossless => {
                WebpConfig { compression_type: CompressionType::Lossless, ..Default::default() }
            }
            CompressionType::Lossy => WebpConfig {
                quality: self.quality.map(|q| q.into()).unwrap_or(DEFAULT_WEBP_QUALITY.into()),
                compression_type: CompressionType::Lossy,
            },
        })
    }

    pub fn avif(&self) -> AvifConfig {
        self.avif.unwrap_or_else(|| AvifConfig {
            quality: self.quality.map(|q| q.into()).unwrap_or(DEFAULT_AVIF_QUALITY.into()),
            ..Default::default()
        })
    }
}
