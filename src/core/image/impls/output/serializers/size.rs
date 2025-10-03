use {
    crate::{ErrorKind, Image, ImageError, ImageFormat, ImageSrc, Result, WithSrc},
    fs_ext::file,
};

impl Image {
    pub fn encoded_size(&self) -> Result<u64> {
        let mut buffer = Vec::new();
        self.encode(&mut buffer, self.format())?;
        Ok(buffer.len() as u64)
    }

    pub fn encoded_size_for(&self, format: ImageFormat) -> Result<u64> {
        let mut buffer = Vec::new();
        self.encode(&mut buffer, format)?;
        Ok(buffer.len() as u64)
    }

    pub fn pixel_data_size(&self) -> Result<u64> {
        let img = self.processed_image()?;
        Ok(img.as_bytes().len() as u64)
    }

    pub fn source_file_size(&self) -> Result<u64> {
        match &self.src {
            ImageSrc::File(path) => Ok(file::size(path).with_src(self.error_src())?),
            _ => Err(ImageError::new(ErrorKind::SourceIsNotFile, self.error_src().cloned())),
        }
    }
}
