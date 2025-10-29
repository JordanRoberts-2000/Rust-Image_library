use crate::{
    constants::{DEFAULT_AVIF_QUALITY, DEFAULT_JPEG_QUALITY, DEFAULT_WEBP_QUALITY},
    encoding::{
        AvifConfig, AvifEncoder, CompressionType, GifConfig, JpegConfig, JpegEncoder, PngConfig,
        PngEncoder, WebpConfig, WebpEncoder,
    },
    image::ImageConfig,
};

impl ImageConfig {
    pub fn jpeg(&self) -> JpegEncoder {
        self.jpeg
            .unwrap_or_else(|| JpegConfig {
                quality: self.quality.map(|q| q.into()).unwrap_or(DEFAULT_JPEG_QUALITY.into()),
                ..Default::default()
            })
            .into()
    }

    pub fn png(&self) -> PngEncoder {
        self.png.unwrap_or_else(PngConfig::default).into()
    }

    pub fn webp(&self) -> WebpEncoder {
        self.webp
            .unwrap_or_else(|| match self.compression {
                CompressionType::Lossless => {
                    WebpConfig { compression_type: CompressionType::Lossless, ..Default::default() }
                }
                CompressionType::Lossy => WebpConfig {
                    quality: self.quality.map(|q| q.into()).unwrap_or(DEFAULT_WEBP_QUALITY.into()),
                    compression_type: CompressionType::Lossy,
                },
            })
            .into()
    }

    pub fn avif(&self) -> AvifEncoder {
        self.avif
            .unwrap_or_else(|| AvifConfig {
                quality: self.quality.map(|q| q.into()).unwrap_or(DEFAULT_AVIF_QUALITY.into()),
                ..Default::default()
            })
            .into()
    }

    pub fn gif(&self) -> GifConfig {
        self.gif.unwrap_or_else(GifConfig::default)
    }
}
