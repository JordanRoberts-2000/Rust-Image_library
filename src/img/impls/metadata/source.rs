use {std::path::PathBuf, url::Url};

use crate::{Img, ImgSrc};

impl Img {
    pub fn source(&self) -> Option<String> {
        match &self.src {
            ImgSrc::File { path } => Some(path.to_string_lossy().into_owned()),
            ImgSrc::Url { url } => Some(url.to_string()),
            ImgSrc::Bytes | ImgSrc::Reader => None,
        }
    }

    pub fn source_path(&self) -> Option<PathBuf> {
        match &self.src {
            ImgSrc::File { path } => Some(path.to_owned()),
            _ => None,
        }
    }

    pub fn source_url(&self) -> Option<Url> {
        match &self.src {
            ImgSrc::Url { url } => Some(url.to_owned()),
            _ => None,
        }
    }
}
