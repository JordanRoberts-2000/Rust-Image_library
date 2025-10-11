use {
    crate::{
        encoding::{AvifColorType, AvifEncoder, EncoderOps, EncodingErrorKind},
        ImageFormat,
    },
    image::{codecs::avif::AvifEncoder as Encoder, ImageEncoder},
    std::io::Write,
};

impl EncoderOps for AvifEncoder {
    type ColorType = AvifColorType;
    const IMAGE_FORMAT: ImageFormat = ImageFormat::Avif;

    fn encode_impl(
        &self, writer: impl Write, bytes: &[u8], w: u32, h: u32, ct: Self::ColorType,
    ) -> Result<(), EncodingErrorKind> {
        Encoder::new_with_speed_quality(writer, self.speed.into(), self.quality.into())
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
            let raw_pixels = raw_pixel_data(ct.into());

            let result = encoder.encode(&mut output, &raw_pixels, width, height, ct);

            assert!(result.is_ok());
            assert!(!output.is_empty(), "Output should contain encoded data");
        }
    }
}
