use {
    crate::{EncodingError, JpegColorType, JpegEncoder},
    image::codecs::jpeg::JpegEncoder as Encoder,
    std::io::Write,
};

impl JpegEncoder {
    pub fn encode(
        &self, writer: impl Write, bytes: &[u8], width: u32, height: u32, color_type: JpegColorType,
    ) -> Result<(), EncodingError> {
        #[cfg(feature = "progressive-jpeg")]
        {
            if self.progressive {
                return self.encode_progressive(writer, bytes, width, height, color_type);
            }
        }

        #[cfg(not(feature = "progressive-jpeg"))]
        if self.progressive {
            warn!(
                "progressive encoding requested, but the 'progressive-jpeg' feature is disabled; \
                 falling back to baseline JPEG."
            );
        }

        Encoder::new_with_quality(writer, self.quality)
            .encode(bytes, width, height, color_type.into())
            .map_err(EncodingError::JpegEncoding)
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
        let encoder = JpegEncoder::new();
        let mut output = Vec::new();
        let (width, height) = MOCK_IMAGE_DIMENSIONS;
        let rgb_data = raw_pixel_data(ColorType::Rgb8);

        let result = encoder.encode(&mut output, &rgb_data, width, height, JpegColorType::Rgb8);

        assert!(result.is_ok());
        assert!(!output.is_empty(), "Output should contain encoded data");
    }

    #[cfg(feature = "progressive-jpeg")]
    #[test]
    fn test_encode_progressive() {
        let encoder = JpegEncoder::progressive();
        let mut output = Vec::new();
        let (width, height) = MOCK_IMAGE_DIMENSIONS;
        let rgb_data = raw_pixel_data(ColorType::Rgb8);

        let result = encoder.encode(&mut output, &rgb_data, width, height, JpegColorType::Rgb8);

        assert!(result.is_ok());
        assert!(!output.is_empty(), "Output should contain encoded data");
    }

    #[cfg(feature = "progressive-jpeg")]
    #[test]
    fn test_encode_different_qualities() {
        let qualities = [1, 25, 50, 75, 100];
        let (width, height) = MOCK_IMAGE_DIMENSIONS;
        let image_data = raw_pixel_data(ColorType::Rgb8);

        for &quality in &qualities {
            let encoder = JpegEncoder::new().with_quality(quality);
            let mut output = Vec::new();

            let result =
                encoder.encode(&mut output, &image_data, width, height, JpegColorType::Rgb8);

            assert!(result.is_ok(), "Failed with quality {}", quality);
            assert!(!output.is_empty(), "No output for quality {}", quality);
        }
    }

    #[cfg(feature = "progressive-jpeg")]
    #[test]
    fn test_encode_progressive_different_qualities() {
        use crate::ColorType;

        let qualities = [1, 25, 50, 75, 100];
        let (width, height) = MOCK_IMAGE_DIMENSIONS;
        let image_data = raw_pixel_data(ColorType::Rgb8);

        for &quality in &qualities {
            let encoder = JpegEncoder::progressive().with_quality(quality);
            let mut output = Vec::new();

            let result =
                encoder.encode(&mut output, &image_data, width, height, JpegColorType::Rgb8);

            assert!(result.is_ok(), "Failed with quality {}", quality);
            assert!(!output.is_empty(), "No output for quality {}", quality);
        }
    }

    #[cfg(feature = "progressive-jpeg")]
    #[test]
    fn test_encode_different_color_types() {
        let encoder = JpegEncoder::new();
        for ct in JpegColorType::iter() {
            let mut output = Vec::new();
            let (width, height) = MOCK_IMAGE_DIMENSIONS;
            let rgb_data = raw_pixel_data((&ct).into());

            let result = encoder.encode(&mut output, &rgb_data, width, height, ct);

            assert!(result.is_ok());
            assert!(!output.is_empty(), "Output should contain encoded data");
        }
    }

    #[cfg(feature = "progressive-jpeg")]
    #[test]
    fn test_encode_progressive_different_color_types() {
        let encoder = JpegEncoder::progressive();
        for ct in JpegColorType::iter() {
            let mut output = Vec::new();
            let (width, height) = MOCK_IMAGE_DIMENSIONS;
            let rgb_data = raw_pixel_data((&ct).into());

            let result = encoder.encode(&mut output, &rgb_data, width, height, ct);

            assert!(result.is_ok());
            assert!(!output.is_empty(), "Output should contain encoded data");
        }
    }

    #[cfg(not(feature = "progressive-jpeg"))]
    #[test]
    fn test_progressive_feature_disabled_falls_back() {
        let encoder = JpegEncoder::progressive();
        let mut output = Vec::new();
        let (width, height) = MOCK_IMAGE_DIMENSIONS;
        let rgb_data = raw_pixel_data(ColorType::Rgb8);

        let result = encoder.encode(&mut output, &rgb_data, width, height, JpegColorType::Rgb8);

        assert!(result.is_ok(), "Should fall back to baseline JPEG");
        assert!(!output.is_empty(), "Output should contain encoded data");
    }
}
