use {image::load_from_memory, std::io::Cursor};

use crate::{ImageFormat, Img, ImgError, Result};

impl Img {
    pub fn png(&mut self) -> Result<&mut Self> {
        if self.format == ImageFormat::Png {
            return Ok(self);
        }

        let mut buffer = Vec::new();
        self.img
            .write_to(&mut Cursor::new(&mut buffer), (ImageFormat::Png).into())
            .map_err(|e| ImgError::Conversion {
                source: e,
                id: self.describe_source(),
                format: ImageFormat::Png,
            })?;

        self.img = load_from_memory(&buffer).map_err(|e| ImgError::Decoding {
            id: self.describe_source(),
            source: e,
            format: ImageFormat::Png,
        })?;

        self.format = ImageFormat::Png;

        Ok(self)
    }
}
