use {
    crate::{
        blocking::{dependencies::FsRepo, traits::FsRepoOps},
        encoders::jpeg::Path,
        utils::decode,
        JpegColorType, JpegEncoder, Result,
    },
    image::{DynamicImage, GenericImageView},
    std::io::Write,
};

impl<'a> JpegEncoder<Path> {
    pub fn write_to(&mut self, writer: impl Write) -> Result<()> {
        self.write_to_internal(writer, FsRepo)
    }

    fn write_to_internal(&mut self, writer: impl Write, fs: impl FsRepoOps) -> Result<()> {
        let (img, width, height, color_type) = self.resolve_metadata(fs)?;
        self.jpeg_encode(writer, img.as_bytes(), width, height, color_type)
    }

    pub fn to_bytes(&mut self) -> Result<Vec<u8>> {
        self.to_bytes_internal(FsRepo)
    }

    fn to_bytes_internal(&mut self, fs: impl FsRepoOps) -> Result<Vec<u8>> {
        let (img, width, height, color_type) = self.resolve_metadata(fs)?;

        let mut output = Vec::new();
        self.jpeg_encode(&mut output, img.as_bytes(), width, height, color_type)?;
        Ok(output)
    }

    fn resolve_metadata(
        &self, fs: impl FsRepoOps,
    ) -> Result<(DynamicImage, u32, u32, JpegColorType)> {
        fs.check_existing_file(&self.input.path)?;
        let img = decode::from_path(&self.input.path)?;

        let (width, height) = img.dimensions();

        let color_type = match &self.color_type {
            Some(ct) => ct.clone(),
            None => img.color().into(),
        };

        Ok((img, width, height, color_type))
    }
}
