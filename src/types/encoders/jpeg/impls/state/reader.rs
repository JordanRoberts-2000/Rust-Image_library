use {
    crate::{
        blocking::{dependencies::IoRepo, traits::IoRepoOps},
        encoders::{
            jpeg::Reader,
            utils::{decode, validate_dimensions},
        },
        JpegColorType, JpegEncoder, Result,
    },
    image::{DynamicImage, GenericImageView},
    std::io::{Read, Write},
};

impl<R: Read> JpegEncoder<Reader<R>> {
    pub fn write_to(self, writer: impl Write) -> Result<()> {
        self.write_to_internal(writer, IoRepo)
    }

    fn write_to_internal(mut self, writer: impl Write, io: impl IoRepoOps) -> Result<()> {
        let (decoded, width, height, color_type) = self.resolve_metadata(io)?;
        self.jpeg_encode(writer, decoded.as_bytes(), width, height, color_type)
    }

    pub fn to_bytes(self) -> Result<Vec<u8>> {
        self.to_bytes_internal(IoRepo)
    }

    fn to_bytes_internal(mut self, io: impl IoRepoOps) -> Result<Vec<u8>> {
        let (decoded, width, height, color_type) = self.resolve_metadata(io)?;

        let mut output = Vec::new();
        self.jpeg_encode(&mut output, decoded.as_bytes(), width, height, color_type)?;
        Ok(output)
    }

    fn resolve_metadata(
        &mut self, io: impl IoRepoOps,
    ) -> Result<(DynamicImage, u32, u32, JpegColorType)> {
        let mut bytes = io.read_to_vec(&mut self.input.reader)?;

        let img = decode(&mut bytes, self.input.format)?;

        let (width, height) = img.dimensions();
        validate_dimensions(width, height)?;

        let color_type = match &self.color_type {
            Some(ct) => ct.clone(),
            None => img.color().into(),
        };

        Ok((img, width, height, color_type))
    }
}
