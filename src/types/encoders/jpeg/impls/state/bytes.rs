use {
    crate::{
        encoders::{
            jpeg::Bytes,
            utils::{decode, validate_dimensions},
            JpegEncoder,
        },
        Result,
    },
    image::GenericImageView,
    std::io::Write,
};

impl<'a> JpegEncoder<Bytes<'a>> {
    pub fn write_to(&mut self, writer: impl Write) -> Result<()> {
        let img = decode(self.input.bytes, self.input.format)?;

        let (width, height) = img.dimensions();
        validate_dimensions(width, height)?;

        let color_type = match &self.color_type {
            Some(ct) => ct.clone(),
            None => img.color().into(),
        };

        self.encode(writer, img.as_bytes(), width, height, color_type)
    }

    pub fn to_bytes(&mut self) -> Result<Vec<u8>> {
        let img = decode(self.input.bytes, self.input.format)?;

        let (width, height) = img.dimensions();
        validate_dimensions(width, height)?;

        let color_type = match &self.color_type {
            Some(ct) => ct.clone(),
            None => img.color().into(),
        };

        let mut output = Vec::new();
        self.encode(&mut output, img.as_bytes(), width, height, color_type)?;

        Ok(output)
    }
}
