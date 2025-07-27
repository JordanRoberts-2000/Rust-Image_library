use {
    crate::{
        encoders::{
            jpeg::Bytes,
            utils::{decode, validate_dimensions},
        },
        JpegColorType, JpegEncoder, Result,
    },
    image::{DynamicImage, GenericImageView},
    std::io::Write,
};

impl<'a> JpegEncoder<Bytes<'a>> {
    pub fn write_to(&mut self, writer: impl Write) -> Result<()> {
        let (img, width, height, color_type) = self.resolve_metadata()?;
        self.jpeg_encode(writer, img.as_bytes(), width, height, color_type)
    }

    pub fn to_bytes(&mut self) -> Result<Vec<u8>> {
        let (img, width, height, color_type) = self.resolve_metadata()?;

        let mut output = Vec::new();
        self.jpeg_encode(&mut output, img.as_bytes(), width, height, color_type)?;

        Ok(output)
    }

    fn resolve_metadata(&self) -> Result<(DynamicImage, u32, u32, JpegColorType)> {
        let img = decode(self.input.bytes, self.input.format)?;

        let (width, height) = img.dimensions();
        validate_dimensions(width, height)?;

        let color_type = match &self.color_type {
            Some(ct) => ct.clone(),
            None => img.color().into(),
        };

        Ok((img, width, height, color_type))
    }
}
