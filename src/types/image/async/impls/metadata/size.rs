use std::fs;

use crate::{Image, ImageError, ImageSize, ImageSrc, IoError, Result};

impl Image {
    pub fn encoded_size(&mut self) -> Result<ImageSize> {
        self.apply_transforms()?;

        let mut buffer = Vec::new();
        self.encode(&mut buffer)?;
        Ok(ImageSize::new(buffer.len()))
    }

    pub fn source_file_size(&mut self) -> Result<ImageSize> {
        match &self.src {
            ImageSrc::File(path) => {
                let metadata =
                    fs::metadata(path).map_err(|e| IoError::MetaData(e, path.to_path_buf()))?;
                Ok(ImageSize::new(metadata.len() as usize))
            }
            _ => Err(ImageError::SourceFileSizeUnavailable),
        }
    }

    pub fn raw_size(&mut self) -> Result<ImageSize> {
        let decoded = self.get_decoded()?;
        let bytes = decoded.as_bytes().len();
        Ok(ImageSize::new(bytes))
    }
}
