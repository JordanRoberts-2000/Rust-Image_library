use {
    crate::{EncodeFormat, ErrorKind, Image, ImageError, ImageSrc, Result, WithSrc},
    fs_ext::file,
};

impl Image {
    pub fn encoded_size(&self) -> Result<u64> {
        let mut buffer = Vec::new();
        self.encode(&mut buffer, self.encoding_format())?;
        Ok(buffer.len() as u64)
    }

    pub fn encoded_size_for(&self, format: EncodeFormat) -> Result<u64> {
        let mut buffer = Vec::new();
        self.encode(&mut buffer, format)?;
        Ok(buffer.len() as u64)
    }

    pub fn decoded_size(&self) -> Result<usize> {
        let decoded = self.decoded();
        Ok(decoded.memory_bytes())
    }

    pub fn source_file_size(&self) -> Result<u64> {
        match &self.src {
            ImageSrc::File(path) => Ok(file::size(path).with_src(self.src())?),
            _ => Err(ImageError::new(ErrorKind::SourceIsNotFile).with_src(self.src())),
        }
    }
}
