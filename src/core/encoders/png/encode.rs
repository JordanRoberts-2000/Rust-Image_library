use {
    crate::{EncodingError, PngColorType, PngEncoder, Result},
    image::{codecs::png::PngEncoder as Encoder, ImageEncoder},
    std::io::Write,
};

impl PngEncoder {
    pub fn encode(
        &self, writer: impl Write, bytes: &[u8], width: u32, height: u32, color_type: PngColorType,
    ) -> Result<()> {
        let encoder = Encoder::new_with_quality(
            writer,
            self.compression_type.into(),
            self.compression_type.filter(),
        );
        encoder
            .write_image(bytes, width, height, color_type.into())
            .map_err(EncodingError::PngEncoding)?;

        Ok(())
    }
}
