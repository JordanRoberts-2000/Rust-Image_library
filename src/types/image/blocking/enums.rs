use {
    image::DynamicImage,
    std::{cell::Ref, path::PathBuf},
};

#[derive(Debug, Clone, PartialEq)]
pub enum ImageData {
    EncodedBytes(Vec<u8>),
    DynamicImage(image::DynamicImage),
    File(PathBuf),
}

pub enum ProcessedImage<'a> {
    Borrowed(Ref<'a, DynamicImage>),
    Owned(DynamicImage),
}

impl<'a> ProcessedImage<'a> {
    pub fn as_ref(&self) -> &DynamicImage {
        match self {
            ProcessedImage::Borrowed(r) => &*r,
            ProcessedImage::Owned(img) => img,
        }
    }
}
