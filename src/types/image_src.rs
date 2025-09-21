use {
    std::{fmt, path::PathBuf},
    url::Url,
};

#[derive(Debug, Clone, PartialEq)]
pub enum ImageSrc {
    File(PathBuf),
    Url(Url),
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
