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
