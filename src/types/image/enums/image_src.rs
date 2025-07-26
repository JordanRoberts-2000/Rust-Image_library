use {std::path::PathBuf, url::Url};

#[derive(Debug, Clone, PartialEq)]
pub enum ImageSrc {
    File(PathBuf),
    Url(Url),
    Base64(String),
    Bytes,
    RawPixels,
    Reader,
}

impl ImageSrc {
    pub fn describe(&self) -> String {
        match self {
            ImageSrc::File(path) => format!("file “{}”", path.display()),
            ImageSrc::Url(url) => format!("URL “{}”", url),
            ImageSrc::Base64(b64) => format!("base64 “{}”", b64),
            ImageSrc::Bytes => "raw bytes".into(),
            ImageSrc::RawPixels => "raw pixels".into(),
            ImageSrc::Reader => "reader".into(),
        }
    }
}
