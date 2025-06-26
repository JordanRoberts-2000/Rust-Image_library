use std::path::{Path, PathBuf};

use url::Url;

use crate::{Image, ImageError};

impl TryFrom<Url> for Image {
    type Error = ImageError;
    fn try_from(url: Url) -> Result<Self, Self::Error> {
        Image::from_url(url)
    }
}

impl TryFrom<Vec<u8>> for Image {
    type Error = ImageError;
    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Image::from_bytes(bytes)
    }
}

impl TryFrom<PathBuf> for Image {
    type Error = ImageError;
    fn try_from(p: PathBuf) -> Result<Self, Self::Error> {
        Image::from_file(p)
    }
}

impl<'a> TryFrom<&'a Path> for Image {
    type Error = ImageError;
    fn try_from(p: &'a Path) -> Result<Self, Self::Error> {
        Image::from_file(p)
    }
}
