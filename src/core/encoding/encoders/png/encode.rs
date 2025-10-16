use {
    crate::{
        encoding::{Encoder, EncodingErrorKind, PngColorType, PngEncoder},
        ImageFormat,
    },
    image::{codecs::png::PngEncoder as ImagePngEncoder, ImageEncoder},
    std::io::Write,
};

impl Encoder for PngEncoder {
    type ColorType = PngColorType;
    const IMAGE_FORMAT: ImageFormat = ImageFormat::Png;

    fn encode_impl(
        &self, writer: &mut dyn Write, bytes: &[u8], w: u32, h: u32, ct: Self::ColorType,
    ) -> Result<(), EncodingErrorKind> {
        let encoder = ImagePngEncoder::new_with_quality(
            writer,
            self.compression_type.into(),
            self.compression_type.filter(),
        );
        encoder
            .write_image(bytes, w, h, ct.into())
            .map_err(|e| EncodingErrorKind::Encode(Box::new(e)))
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            encoding::ColorType,
            test_utils::{raw_pixel_data, MOCK_IMAGE_DIMENSIONS},
        },
        strum::IntoEnumIterator,
    };

    #[test]
    fn test_encode() {
        let encoder = PngEncoder::new();
        let mut output = Vec::new();
        let (width, height) = MOCK_IMAGE_DIMENSIONS;
        let rgb_data = raw_pixel_data(ColorType::Rgb8);

        let result = encoder.encode(&mut output, &rgb_data, width, height, PngColorType::Rgb8);

        assert!(result.is_ok());
        assert!(!output.is_empty(), "Output should contain encoded data");
    }

    #[test]
    fn test_encode_fast() {
        let encoder = PngEncoder::fast();
        let mut output = Vec::new();
        let (width, height) = MOCK_IMAGE_DIMENSIONS;
        let rgb_data = raw_pixel_data(ColorType::Rgb8);

        let result = encoder.encode(&mut output, &rgb_data, width, height, PngColorType::Rgb8);

        assert!(result.is_ok());
        assert!(!output.is_empty(), "Output should contain encoded data");
    }

    #[test]
    fn test_encode_best_compression() {
        let encoder = PngEncoder::best_compression();
        let mut output = Vec::new();
        let (width, height) = MOCK_IMAGE_DIMENSIONS;
        let rgb_data = raw_pixel_data(ColorType::Rgb8);

        let result = encoder.encode(&mut output, &rgb_data, width, height, PngColorType::Rgb8);

        assert!(result.is_ok());
        assert!(!output.is_empty(), "Output should contain encoded data");
    }

    #[test]
    fn test_encode_different_color_types() {
        let encoder = PngEncoder::new();
        for ct in PngColorType::iter() {
            let mut output = Vec::new();
            let (width, height) = MOCK_IMAGE_DIMENSIONS;
            let rgb_data = raw_pixel_data(ct.into());

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
            let (width, height) = MOCK_IMAGE_DIMENSIONS;
            let rgb_data = raw_pixel_data(ct.into());

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
            let (width, height) = MOCK_IMAGE_DIMENSIONS;
            let rgb_data = raw_pixel_data(ct.into());

            let result = encoder.encode(&mut output, &rgb_data, width, height, ct);

            assert!(result.is_ok());
            assert!(!output.is_empty(), "Output should contain encoded data");
        }
    }
}
