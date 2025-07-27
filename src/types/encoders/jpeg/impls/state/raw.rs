use {
    crate::{
        encoders::{jpeg::Raw, utils::validate_dimensions},
        JpegColorType, JpegEncoder, Result, ValidationError,
    },
    std::io::Write,
};

impl<'a> JpegEncoder<Raw<'a>> {
    pub fn write_to(&mut self, writer: impl Write) -> Result<()> {
        let (width, height, color_type) = self.resolve_metadata()?;
        self.jpeg_encode(writer, self.input.bytes, width, height, color_type)
    }

    pub fn to_bytes(&mut self) -> Result<Vec<u8>> {
        let (width, height, color_type) = self.resolve_metadata()?;

        let mut output = Vec::new();
        self.jpeg_encode(&mut output, self.input.bytes, width, height, color_type)?;

        Ok(output)
    }

    fn resolve_metadata(&self) -> Result<(u32, u32, JpegColorType)> {
        if self.input.bytes.is_empty() {
            return Err(ValidationError::EmptyByteArray.into());
        }

        let (width, height) = (self.input.width, self.input.height);
        validate_dimensions(width, height)?;

        let color_type = self.color_type.as_ref().cloned().unwrap_or_default();

        Ok((width, height, color_type))
    }
}
