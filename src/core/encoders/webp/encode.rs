use {
    crate::{CompressionType, EncodingError, Result, WebPColorType, WebPEncoder},
    image::codecs::webp::WebPEncoder as LosslessEncoder,
    std::io::Write,
    webp::Encoder as LossyEncoder,
};

impl WebPEncoder {
    pub fn encode(
        &self, mut writer: impl Write, bytes: &[u8], width: u32, height: u32,
        color_type: WebPColorType,
    ) -> Result<()> {
        match self.compression_type {
            CompressionType::Lossy => {
                let encoder =
                    LossyEncoder::new(bytes, WebPColorType::from(color_type).into(), width, height);
                let encoded = encoder.encode(self.quality as f32).to_vec();
                writer.write_all(&encoded)?;
            }
            CompressionType::Lossless => {
                let encoder = LosslessEncoder::new_lossless(writer);
                encoder
                    .encode(bytes, width, height, color_type.into())
                    .map_err(EncodingError::WebPLosslessEncoding)?;
            }
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::test_utils::{create_image_data, create_rgb_data},
        strum::IntoEnumIterator,
    };

    #[test]
    fn test_encode_lossy() {
        let encoder = WebPEncoder::lossy(80);
        let mut output = Vec::new();
        let (width, height) = (12, 12);
        let rgb_data = create_rgb_data(width, height);

        let result = encoder.encode(&mut output, &rgb_data, width, height, WebPColorType::Rgb8);

        assert!(result.is_ok());
        assert!(!output.is_empty(), "Output should contain encoded data");
    }

    #[test]
    fn test_encode_lossless() {
        let encoder = WebPEncoder::lossless();
        let mut output = Vec::new();
        let (width, height) = (12, 12);
        let rgb_data = create_rgb_data(width, height);

        let result = encoder.encode(&mut output, &rgb_data, width, height, WebPColorType::Rgb8);

        assert!(result.is_ok());
        assert!(!output.is_empty(), "Output should contain encoded data");
    }

    #[test]
    fn test_encode_lossy_different_qualities() {
        let qualities = [1, 25, 50, 75, 100];
        let (width, height) = (12, 12);
        let image_data = create_rgb_data(width, height);

        for &quality in &qualities {
            let encoder = WebPEncoder::lossy(quality);
            let mut output = Vec::new();

            let result =
                encoder.encode(&mut output, &image_data, width, height, WebPColorType::Rgb8);

            assert!(result.is_ok(), "Failed with quality {}", quality);
            assert!(!output.is_empty(), "No output for quality {}", quality);
        }
    }

    #[test]
    fn test_encode_lossy_different_color_types() {
        let encoder = WebPEncoder::lossy(80);
        for ct in WebPColorType::iter() {
            let mut output = Vec::new();
            let (width, height) = (12, 12);
            let rgb_data = create_image_data(width, height, ct.channels());

            let result = encoder.encode(&mut output, &rgb_data, width, height, ct);

            assert!(result.is_ok());
            assert!(!output.is_empty(), "Output should contain encoded data");
        }
    }

    #[test]
    fn test_encode_lossless_different_color_types() {
        let encoder = WebPEncoder::lossless();
        for ct in WebPColorType::iter() {
            let mut output = Vec::new();
            let (width, height) = (12, 12);
            let rgb_data = create_image_data(width, height, ct.channels());

            let result = encoder.encode(&mut output, &rgb_data, width, height, ct);

            assert!(result.is_ok());
            assert!(!output.is_empty(), "Output should contain encoded data");
        }
    }
}
