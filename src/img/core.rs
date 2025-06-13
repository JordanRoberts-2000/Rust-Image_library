use crate::{ImageFormat, ImgSrc};

#[derive(Debug)]
pub struct Img {
    pub(super) img: image::DynamicImage,
    pub(super) src: ImgSrc,
    pub height: u32,
    pub width: u32,
    pub aspect_ratio: f32,
    pub format: ImageFormat,
    pub(super) size_bytes: usize,
}
