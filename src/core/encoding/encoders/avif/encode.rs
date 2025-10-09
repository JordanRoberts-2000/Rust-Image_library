use {
    crate::{AvifColorType, AvifEncoder, EncodingError},
    image::{codecs::avif::AvifEncoder as Encoder, ImageEncoder},
    std::io::Write,
};

impl AvifEncoder {
    pub fn encode(
        &self, writer: impl Write, bytes: &[u8], width: u32, height: u32, color_type: AvifColorType,
    ) -> Result<(), EncodingError> {
        Encoder::new_with_speed_quality(writer, self.speed, self.quality)
            .write_image(bytes, width, height, color_type.into())
            .map_err(|err| EncodingError::AvifEncoding { err })
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            test_utils::{raw_pixel_data, MOCK_IMAGE_DIMENSIONS},
            ColorType,
        },
        strum::IntoEnumIterator,
    };

    #[test]
    fn test_encode() {
        let encoder = AvifEncoder::new();
        let mut output = Vec::new();
        let (width, height) = MOCK_IMAGE_DIMENSIONS;
        let rgb_data = raw_pixel_data(ColorType::Rgb8);

        let result = encoder.encode(&mut output, &rgb_data, width, height, AvifColorType::Rgb8);

        assert!(result.is_ok());
        assert!(!output.is_empty(), "Output should contain encoded data");
    }

    #[test]
    fn test_encode_different_color_types() {
        let encoder = AvifEncoder::new();
        for ct in AvifColorType::iter() {
            let mut output = Vec::new();
            let (width, height) = MOCK_IMAGE_DIMENSIONS;
            let raw_pixels = raw_pixel_data((&ct).into());

            let result = encoder.encode(&mut output, &raw_pixels, width, height, ct);

            assert!(result.is_ok());
            assert!(!output.is_empty(), "Output should contain encoded data");
        }
    }
}
