use crate::{Image, ImageError, ImageFormat};

impl Image {
    pub fn extention(&self) -> Result<&'static str, ImageError> {
        match self.format {
            ImageFormat::Jpeg => Ok("jpg"),
            ImageFormat::WebP => Ok("webp"),
            ImageFormat::Png => Ok("png"),
        }
    }
}
