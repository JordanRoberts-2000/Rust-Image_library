use std::path::{Path, PathBuf};

use url::Url;

use crate::{Img, ImgError};

impl TryFrom<Url> for Img {
    type Error = ImgError;
    fn try_from(url: Url) -> Result<Self, Self::Error> {
        Img::from_url(url)
    }
}

impl TryFrom<Vec<u8>> for Img {
    type Error = ImgError;
    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Img::from_bytes(bytes)
    }
}

impl TryFrom<PathBuf> for Img {
    type Error = ImgError;
    fn try_from(p: PathBuf) -> Result<Self, Self::Error> {
        Img::from_file(p)
    }
}

impl<'a> TryFrom<&'a Path> for Img {
    type Error = ImgError;
    fn try_from(p: &'a Path) -> Result<Self, Self::Error> {
        Img::from_file(p)
    }
}
