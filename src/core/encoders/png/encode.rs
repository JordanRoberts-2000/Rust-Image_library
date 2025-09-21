use {
    crate::{EncodingError, PngColorType, PngEncoder},
    image::{codecs::png::PngEncoder as Encoder, ImageEncoder},
    std::io::Write,
};

impl PngEncoder {
    pub fn encode(
        &self, writer: impl Write, bytes: &[u8], width: u32, height: u32, color_type: PngColorType,
    ) -> Result<(), EncodingError> {
        let encoder = Encoder::new_with_quality(
            writer,
            self.compression_type.into(),
            self.compression_type.filter(),
        );
        encoder
            .write_image(bytes, width, height, color_type.into())
            .map_err(EncodingError::PngEncoding)
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::test_utils::{create_image_data, create_rgb8_data},
        strum::IntoEnumIterator,
    };

    #[test]
    fn test_encode() {
        let encoder = PngEncoder::new();
        let mut output = Vec::new();
        let (width, height) = (12, 12);
        let rgb_data = create_rgb8_data(width, height);

        let result = encoder.encode(&mut output, &rgb_data, width, height, PngColorType::Rgb8);

        assert!(result.is_ok());
        assert!(!output.is_empty(), "Output should contain encoded data");
    }

    #[test]
    fn test_encode_fast() {
        let encoder = PngEncoder::fast();
        let mut output = Vec::new();
        let (width, height) = (12, 12);
        let rgb_data = create_rgb8_data(width, height);

        let result = encoder.encode(&mut output, &rgb_data, width, height, PngColorType::Rgb8);

        assert!(result.is_ok());
        assert!(!output.is_empty(), "Output should contain encoded data");
    }

    #[test]
    fn test_encode_best_compression() {
        let encoder = PngEncoder::best_compression();
        let mut output = Vec::new();
        let (width, height) = (12, 12);
        let rgb_data = create_rgb8_data(width, height);

        let result = encoder.encode(&mut output, &rgb_data, width, height, PngColorType::Rgb8);

        assert!(result.is_ok());
        assert!(!output.is_empty(), "Output should contain encoded data");
    }

    #[test]
    fn test_encode_different_color_types() {
        let encoder = PngEncoder::new();
        for ct in PngColorType::iter() {
            let mut output = Vec::new();
            let (width, height) = (12, 12);
            let rgb_data = create_image_data(width, height, (&ct).into());

            let result = encoder.encode(&mut output, &rgb_data, width, height, ct);

            assert!(result.is_ok());
            assert!(!output.is_empty(), "Output should contain encoded data");
        }
    }

    #[test]
    fn test_encode_fast_different_color_types() {
        let encoder = PngEncoder::fast();
        for ct in PngColorType::iter() {
            let mut output = Vec::new();
            let (width, height) = (12, 12);
            let rgb_data = create_image_data(width, height, (&ct).into());

            let result = encoder.encode(&mut output, &rgb_data, width, height, ct);

            assert!(result.is_ok());
            assert!(!output.is_empty(), "Output should contain encoded data");
        }
    }

    #[test]
    fn test_encode_best_compression_different_color_types() {
        let encoder = PngEncoder::best_compression();
        for ct in PngColorType::iter() {
            let mut output = Vec::new();
            let (width, height) = (12, 12);
            let rgb_data = create_image_data(width, height, (&ct).into());

            let result = encoder.encode(&mut output, &rgb_data, width, height, ct);

            assert!(result.is_ok());
            assert!(!output.is_empty(), "Output should contain encoded data");
        }
    }
}
