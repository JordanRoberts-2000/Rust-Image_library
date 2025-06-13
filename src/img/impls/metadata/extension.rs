use crate::{Img, ImgError};

use image::ImageFormat;

impl Img {
    pub fn extention(&self) -> Result<&'static str, ImgError> {
        match self.format {
            ImageFormat::Jpeg => Ok("jpg"),
            ImageFormat::WebP => Ok("webp"),
            ImageFormat::Png => Ok("png"),
            _ => Err(ImgError::ExtensionInvalid),
        }
    }
}
