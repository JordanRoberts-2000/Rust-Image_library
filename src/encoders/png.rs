use {
    image::{
        codecs::png::{CompressionType, FilterType, PngEncoder as Encoder},
        load_from_memory, ColorType, DynamicImage, GenericImageView, ImageEncoder, ImageError,
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
        let (decoded, width, height, color_type) = Self::decode_bytes(bytes)?;
        self.write_encoded(writer, &decoded, width, height, color_type)
    }

    pub fn write_from_raw_to(
        &self,
        writer: impl Write,
        bytes: &[u8],
        width: u32,
        height: u32,
        color_type: ColorType,
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
        color_type: ColorType,
    ) -> Result<(), PngEncoderError> {
        let encoder = Encoder::new_with_quality(writer, self.compression, self.filter);
        encoder
            .write_image(decoded, width, height, color_type.into())
            .map_err(PngEncoderError::Encoding)
    }

    fn decode_bytes(bytes: &[u8]) -> Result<(Vec<u8>, u32, u32, ColorType), PngEncoderError> {
        let img = load_from_memory(bytes).map_err(PngEncoderError::ImageLoad)?;
        Self::extract_raw_data(img)
    }

    fn extract_raw_data(
        img: DynamicImage,
    ) -> Result<(Vec<u8>, u32, u32, ColorType), PngEncoderError> {
        let (width, height) = img.dimensions();
        Self::validate_dimensions(width, height)?;

        let (decoded, color_type) = match img {
            DynamicImage::ImageLuma8(gray) => (gray.into_raw(), ColorType::L8),
            DynamicImage::ImageLumaA8(gray_alpha) => (gray_alpha.into_raw(), ColorType::La8),
            DynamicImage::ImageRgb8(rgb) => (rgb.into_raw(), ColorType::Rgb8),
            DynamicImage::ImageRgba8(rgba) => (rgba.into_raw(), ColorType::Rgba8),

            DynamicImage::ImageLuma16(gray16) => (
                gray16
                    .into_raw()
                    .into_iter()
                    .flat_map(|p| p.to_be_bytes())
                    .collect(),
                ColorType::L16,
            ),
            DynamicImage::ImageLumaA16(gray_alpha16) => (
                gray_alpha16
                    .into_raw()
                    .into_iter()
                    .flat_map(|p| p.to_be_bytes())
                    .collect(),
                ColorType::La16,
            ),
            DynamicImage::ImageRgb16(rgb16) => (
                rgb16
                    .into_raw()
                    .into_iter()
                    .flat_map(|p| p.to_be_bytes())
                    .collect(),
                ColorType::Rgb16,
            ),
            DynamicImage::ImageRgba16(rgba16) => (
                rgba16
                    .into_raw()
                    .into_iter()
                    .flat_map(|p| p.to_be_bytes())
                    .collect(),
                ColorType::Rgba16,
            ),

            DynamicImage::ImageRgb32F(_) | DynamicImage::ImageRgba32F(_) => {
                // Convert float images to 8-bit for PNG compatibility
                let rgb8 = img.into_rgb8();
                (rgb8.into_raw(), ColorType::Rgb8)
            }

            _ => {
                // Fallback: convert unknown types to RGB8
                let rgb8 = img.into_rgb8();
                (rgb8.into_raw(), ColorType::Rgb8)
            }
        };

        Ok((decoded, width, height, color_type))
    }
}
