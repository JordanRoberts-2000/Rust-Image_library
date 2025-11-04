use {
    std::{
        fmt,
        path::{Path, PathBuf},
    },
    url::Url,
};

#[derive(Debug, Clone, PartialEq)]
pub enum ImageOrigin {
    File(PathBuf),
    Url(String),
    Base64(String),
    Bytes,
    RawPixels,
    Reader,
}

impl fmt::Display for ImageOrigin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImageOrigin::File(path) => write!(f, "file \"{}\"", path.display()),
            ImageOrigin::Url(url) => write!(f, "URL \"{}\"", url),
            ImageOrigin::Base64(sample) => write!(f, "base64 \"{}...\"", sample),
            ImageOrigin::Bytes => f.write_str("encoded bytes"),
            ImageOrigin::RawPixels => f.write_str("raw pixels"),
            ImageOrigin::Reader => f.write_str("reader"),
        }
    }
}

impl From<PathBuf> for ImageOrigin {
    #[inline]
    fn from(path: PathBuf) -> Self {
        ImageOrigin::File(path)
    }
}

impl From<&Path> for ImageOrigin {
    #[inline]
    fn from(path: &Path) -> Self {
        ImageOrigin::File(path.to_path_buf())
    }
}

impl From<Url> for ImageOrigin {
    #[inline]
    fn from(url: Url) -> Self {
        ImageOrigin::Url(url.to_string())
    }
}

impl From<&Url> for ImageOrigin {
    #[inline]
    fn from(url: &Url) -> Self {
        ImageOrigin::Url(url.to_string())
    }
}
