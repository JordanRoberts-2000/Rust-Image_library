use {
    image::{
        codecs::png::{CompressionType, FilterType, PngEncoder as Encoder},
        load_from_memory, ColorType, ImageEncoder, ImageError,
    },
    std::io::Write,
};

#[derive(thiserror::Error, Debug)]
pub enum PngEncoderError {
    #[error("Failed to load image from memory")]
    ImageLoad(ImageError),

    #[error("Failed to encode PNG")]
    Encoding(ImageError),

    #[error("Invalid image dimensions: width={width}, height={height}")]
    InvalidDimensions { width: u32, height: u32 },
}

pub struct PngEncoder {
    compression: CompressionType,
    filter: FilterType,
}

impl Default for PngEncoder {
    fn default() -> Self {
        Self::new()
    }
}

impl PngEncoder {
    pub fn new() -> Self {
        Self {
            compression: CompressionType::Default,
            filter: FilterType::Adaptive,
        }
    }

    pub fn best_compression() -> Self {
        Self {
            compression: CompressionType::Best,
            filter: FilterType::Adaptive,
        }
    }

    pub fn fast() -> Self {
        Self {
            compression: CompressionType::Fast,
            filter: FilterType::NoFilter,
        }
    }

    pub fn with_compression(mut self, compression: CompressionType) -> Self {
        self.compression = compression;
        self
    }

    pub fn with_filter(mut self, filter: FilterType) -> Self {
        self.filter = filter;
        self
    }

    pub fn encode(&self, bytes: &[u8]) -> Result<Vec<u8>, PngEncoderError> {
        let mut buffer = Vec::new();
        self.write_to(&mut buffer, bytes)?;
        Ok(buffer)
    }

    pub fn encode_from_raw(
        &self,
        bytes: &[u8],
        width: u32,
        height: u32,
    ) -> Result<Vec<u8>, PngEncoderError> {
        let mut buffer = Vec::new();
        self.write_encoded(&mut buffer, &bytes, width, height)?;
        Ok(buffer)
    }

    pub fn write_to(&self, writer: impl Write, bytes: &[u8]) -> Result<(), PngEncoderError> {
        let (decoded, width, height) = Self::decode_bytes(bytes)?;
        self.write_encoded(writer, &decoded, width, height)
    }

    pub fn write_from_raw_to(
        &self,
        writer: impl Write,
        bytes: &[u8],
        width: u32,
        height: u32,
    ) -> Result<(), PngEncoderError> {
        self.write_encoded(writer, bytes, width, height)
    }

    fn validate_dimensions(width: u32, height: u32) -> Result<(), PngEncoderError> {
        if width == 0 || height == 0 {
            return Err(PngEncoderError::InvalidDimensions { width, height });
        }
        Ok(())
    }

    fn write_encoded(
        &self,
        writer: impl Write,
        decoded: &[u8],
        width: u32,
        height: u32,
    ) -> Result<(), PngEncoderError> {
        let encoder = Encoder::new_with_quality(writer, self.compression, self.filter);
        encoder
            .write_image(decoded, width, height, ColorType::Rgb8.into())
            .map_err(PngEncoderError::Encoding)
    }

    fn decode_bytes(bytes: &[u8]) -> Result<(Vec<u8>, u32, u32), PngEncoderError> {
        let img = load_from_memory(bytes).map_err(PngEncoderError::ImageLoad)?;

        let rgb = img.into_rgb8();
        let (width, height) = rgb.dimensions();

        Self::validate_dimensions(width, height)?;

        Ok((rgb.into_raw(), width, height))
    }
}
