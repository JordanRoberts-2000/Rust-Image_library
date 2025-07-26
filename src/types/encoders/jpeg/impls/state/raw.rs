use {
    crate::{
        encoders::{jpeg::Raw, utils::validate_dimensions, JpegEncoder},
        Result,
    },
    std::io::Write,
};

impl<'a> JpegEncoder<Raw<'a>> {
    pub fn write_to(&mut self, writer: impl Write) -> Result<()> {
        let (width, height) = (self.input.width, self.input.height);
        validate_dimensions(width, height)?;

        let color_type = self.color_type.as_ref().cloned().unwrap_or_default();

        self.encode(writer, self.input.bytes, width, height, color_type)
    }

    pub fn to_bytes(&mut self) -> Result<Vec<u8>> {
        let (width, height) = (self.input.width, self.input.height);
        validate_dimensions(width, height)?;

        let color_type = self.color_type.as_ref().cloned().unwrap_or_default();

        let mut output = Vec::new();
        self.encode(&mut output, self.input.bytes, width, height, color_type)?;

        Ok(output)
    }
}
