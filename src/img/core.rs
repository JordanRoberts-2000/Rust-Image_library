use std::path::PathBuf;

use crate::enums::ImgSrc;

#[derive(Debug)]
pub struct Img {
    pub(super) img: image::DynamicImage,
    pub(super) src: ImgSrc,
    pub target_path: PathBuf,
    pub height: u32,
    pub width: u32,
    pub aspect_ratio: f32,
    pub format: image::ImageFormat,
    pub size_bytes: usize,
}
