use {
    crate::{Image, ImageError, ImageSrc},
    std::path::{Path, PathBuf},
    url::Url,
};

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

impl TryFrom<ImageSrc> for Image {
    type Error = ImageError;
    fn try_from(src: ImageSrc) -> Result<Self, Self::Error> {
        match src {
            ImageSrc::File(path) => Image::from_file(path),
            ImageSrc::Url(url) => Image::from_url(url),
        }
    }
}
