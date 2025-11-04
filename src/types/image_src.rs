use {
    std::path::{Path, PathBuf},
    url::Url,
};

pub enum ImageSrc {
    File(PathBuf),
    Url(String),
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
