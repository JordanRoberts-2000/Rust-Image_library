use {
    crate::{AvifColorType, AvifEncoder, EncodingError, Result},
    image::{codecs::avif::AvifEncoder as Encoder, ImageEncoder},
    std::io::Write,
};

impl AvifEncoder {
    pub fn encode(
        &self, writer: impl Write, bytes: &[u8], width: u32, height: u32, color_type: AvifColorType,
    ) -> Result<()> {
        Encoder::new_with_speed_quality(writer, self.speed, self.quality)
            .write_image(bytes, width, height, color_type.into())
            .map_err(|err| EncodingError::AvifEncoding { err })?;

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
        let encoder = AvifEncoder::new();
        let mut output = Vec::new();
        let (width, height) = (12, 12);
        let rgb_data = create_rgb8_data(width, height);

        let result = encoder.encode(&mut output, &rgb_data, width, height, AvifColorType::Rgb8);

        assert!(result.is_ok());
        assert!(!output.is_empty(), "Output should contain encoded data");
    }

    #[test]
    fn test_encode_different_color_types() {
        let encoder = AvifEncoder::new();
        for ct in AvifColorType::iter() {
            let mut output = Vec::new();
            let (width, height) = (12, 12);
            let rgb_data = create_image_data(width, height, (&ct).into());

            let result = encoder.encode(&mut output, &rgb_data, width, height, ct);

            assert!(result.is_ok());
            assert!(!output.is_empty(), "Output should contain encoded data");
        }
    }
}
