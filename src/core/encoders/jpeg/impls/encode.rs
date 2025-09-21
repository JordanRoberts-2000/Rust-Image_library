use {
    crate::{EncodingError, JpegColorType, JpegEncoder, Result},
    image::codecs::jpeg::JpegEncoder as Encoder,
    std::io::Write,
};

impl JpegEncoder {
    pub fn encode(
        &self, writer: impl Write, bytes: &[u8], width: u32, height: u32, color_type: JpegColorType,
    ) -> Result<()> {
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
            .map_err(EncodingError::JpegEncoding)?;

        Ok(())
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
        let encoder = JpegEncoder::new();
        let mut output = Vec::new();
        let (width, height) = (12, 12);
        let rgb_data = create_rgb8_data(width, height);

        let result = encoder.encode(&mut output, &rgb_data, width, height, JpegColorType::Rgb8);

        assert!(result.is_ok());
        assert!(!output.is_empty(), "Output should contain encoded data");
    }

    #[cfg(feature = "progressive-jpeg")]
    #[test]
    fn test_encode_progressive() {
        let encoder = JpegEncoder::progressive();
        let mut output = Vec::new();
        let (width, height) = (12, 12);
        let rgb_data = create_rgb8_data(width, height);

        let result = encoder.encode(&mut output, &rgb_data, width, height, JpegColorType::Rgb8);

        assert!(result.is_ok());
        assert!(!output.is_empty(), "Output should contain encoded data");
    }

    #[cfg(feature = "progressive-jpeg")]
    #[test]
    fn test_encode_different_qualities() {
        let qualities = [1, 25, 50, 75, 100];
        let (width, height) = (12, 12);
        let image_data = create_rgb8_data(width, height);

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
        let qualities = [1, 25, 50, 75, 100];
        let (width, height) = (12, 12);
        let image_data = create_rgb8_data(width, height);

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
            let (width, height) = (12, 12);
            let rgb_data = create_image_data(width, height, (&ct).into());

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
            let (width, height) = (12, 12);
            let rgb_data = create_image_data(width, height, (&ct).into());

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
        let (width, height) = (12, 12);
        let rgb_data = create_rgb8_data(width, height);

        let result = encoder.encode(&mut output, &rgb_data, width, height, JpegColorType::Rgb8);

        assert!(result.is_ok(), "Should fall back to baseline JPEG");
        assert!(!output.is_empty(), "Output should contain encoded data");
    }
}
