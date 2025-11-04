use {
    crate::{image::ImageOrigin, EncodeFormat, ErrorKind, Image, ImageError, Result, WithOrigin},
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
        match &self.origin {
            ImageOrigin::File(path) => Ok(file::size(path).with_origin(self.origin())?),
            _ => Err(ImageError::new(ErrorKind::SourceIsNotFile).with_origin(self.origin())),
        }
    }
}
