use {
    crate::{
        encoding::{CompressionType, Encoder, EncodingErrorKind, WebpColorType, WebpEncoder},
        ImageFormat,
    },
    image::codecs::webp::WebPEncoder as LosslessEncoder,
    std::io::Write,
    webp::Encoder as LossyEncoder,
};

impl Encoder for WebpEncoder {
    type ColorType = WebpColorType;
    const IMAGE_FORMAT: ImageFormat = ImageFormat::Webp;

    fn encode_impl(
        &self, writer: &mut dyn Write, bytes: &[u8], w: u32, h: u32, ct: Self::ColorType,
    ) -> Result<(), EncodingErrorKind> {
        match self.compression_type {
            CompressionType::Lossy => {
                let encoder = LossyEncoder::new(bytes, WebpColorType::from(ct).into(), w, h);
                let encoded = encoder.encode(self.quality.into()).to_vec();
                writer.write_all(&encoded)?;
            }
            CompressionType::Lossless => {
                let encoder = LosslessEncoder::new_lossless(writer);
                encoder
                    .encode(bytes, w, h, ct.into())
                    .map_err(|e| EncodingErrorKind::Encode(Box::new(e)))?;
            }
        };

        Ok(())
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
    fn test_encode_lossy() {
        let encoder = WebpEncoder::lossy(80);
        let mut output = Vec::new();
        let (width, height) = MOCK_IMAGE_DIMENSIONS;
        let rgb_data = raw_pixel_data(ColorType::Rgb8);

        let result = encoder.encode(&mut output, &rgb_data, width, height, WebpColorType::Rgb8);

        assert!(result.is_ok());
        assert!(!output.is_empty(), "Output should contain encoded data");
    }

    #[test]
    fn test_encode_lossless() {
        let encoder = WebpEncoder::lossless();
        let mut output = Vec::new();
        let (width, height) = MOCK_IMAGE_DIMENSIONS;
        let rgb_data = raw_pixel_data(ColorType::Rgb8);

        let result = encoder.encode(&mut output, &rgb_data, width, height, WebpColorType::Rgb8);

        assert!(result.is_ok());
        assert!(!output.is_empty(), "Output should contain encoded data");
    }

    #[test]
    fn test_encode_lossy_different_qualities() {
        let qualities = [1, 25, 50, 75, 100];
        let (width, height) = MOCK_IMAGE_DIMENSIONS;
        let image_data = raw_pixel_data(ColorType::Rgb8);

        for &quality in &qualities {
            let encoder = WebpEncoder::lossy(quality);
            let mut output = Vec::new();

            let result =
                encoder.encode(&mut output, &image_data, width, height, WebpColorType::Rgb8);

            assert!(result.is_ok(), "Failed with quality {}", quality);
            assert!(!output.is_empty(), "No output for quality {}", quality);
        }
    }

    #[test]
    fn test_encode_lossy_different_color_types() {
        let encoder = WebpEncoder::lossy(80);
        for ct in WebpColorType::iter() {
            let mut output = Vec::new();
            let (width, height) = MOCK_IMAGE_DIMENSIONS;
            let rgb_data = raw_pixel_data(ct.into());

            let result = encoder.encode(&mut output, &rgb_data, width, height, ct);

            assert!(result.is_ok());
            assert!(!output.is_empty(), "Output should contain encoded data");
        }
    }

    #[test]
    fn test_encode_lossless_different_color_types() {
        let encoder = WebpEncoder::lossless();
        for ct in WebpColorType::iter() {
            let mut output = Vec::new();
            let (width, height) = MOCK_IMAGE_DIMENSIONS;
            let rgb_data = raw_pixel_data(ct.into());

            let result = encoder.encode(&mut output, &rgb_data, width, height, ct);

            assert!(result.is_ok());
            assert!(!output.is_empty(), "Output should contain encoded data");
        }
    }
}
