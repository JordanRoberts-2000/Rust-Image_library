use crate::{ImageFormat, Img, ImgError};

impl Img {
    pub fn extention(&self) -> Result<&'static str, ImgError> {
        match self.format {
            ImageFormat::Jpeg => Ok("jpg"),
            ImageFormat::WebP => Ok("webp"),
            ImageFormat::Png => Ok("png"),
        }
    }
}
