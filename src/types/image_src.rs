use {
    std::{
        fmt,
        path::{Path, PathBuf},
    },
    url::Url,
};

#[derive(Debug, Clone, PartialEq)]
pub enum ImageSrc {
    File(PathBuf),
    Url(String),
    Base64(String),
    Bytes,
    RawPixels,
    Reader,
}

impl fmt::Display for ImageSrc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImageSrc::File(path) => write!(f, "file \"{}\"", path.display()),
            ImageSrc::Url(url) => write!(f, "URL \"{}\"", url),
            ImageSrc::Base64(sample) => write!(f, "base64 \"{}...\"", sample),
            ImageSrc::Bytes => f.write_str("encoded bytes"),
            ImageSrc::RawPixels => f.write_str("raw pixels"),
            ImageSrc::Reader => f.write_str("reader"),
        }
    }
}

impl From<PathBuf> for ImageSrc {
    #[inline]
    fn from(path: PathBuf) -> Self {
        ImageSrc::File(path)
    }
}

impl From<&Path> for ImageSrc {
    #[inline]
    fn from(path: &Path) -> Self {
        ImageSrc::File(path.to_path_buf())
    }
}

impl From<Url> for ImageSrc {
    #[inline]
    fn from(url: Url) -> Self {
        ImageSrc::Url(url.to_string())
    }
}

impl From<&Url> for ImageSrc {
    #[inline]
    fn from(url: &Url) -> Self {
        ImageSrc::Url(url.to_string())
    }
}
