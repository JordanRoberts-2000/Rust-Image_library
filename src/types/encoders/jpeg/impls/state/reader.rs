use {
    crate::{
        encoders::{
            jpeg::Reader,
            utils::{decode, validate_dimensions},
            JpegEncoder,
        },
        IoError, Result,
    },
    image::GenericImageView,
    std::io::{Read, Write},
};

impl<R: Read> JpegEncoder<Reader<R>> {
    pub fn write_to(mut self, writer: impl Write) -> Result<()> {
        let mut buffer = Vec::new();
        self.input.reader.read_to_end(&mut buffer).map_err(IoError::ReadStream)?;

        let img = decode(&mut buffer, self.input.format)?;

        let (width, height) = img.dimensions();
        validate_dimensions(width, height)?;

        let color_type = match &self.color_type {
            Some(ct) => ct.clone(),
            None => img.color().into(),
        };

        self.encode(writer, img.as_bytes(), width, height, color_type)
    }

    pub fn to_bytes(mut self) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        self.input.reader.read_to_end(&mut buffer).map_err(IoError::ReadStream)?;

        let img = decode(&mut buffer, self.input.format)?;

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
