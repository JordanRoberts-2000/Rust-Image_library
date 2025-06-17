use {std::path::PathBuf, url::Url};

use crate::{Image, ImageSrc};

impl Image {
    pub fn source(&self) -> Option<String> {
        match &self.src {
            ImageSrc::File { path } => Some(path.to_string_lossy().into_owned()),
            ImageSrc::Url { url } => Some(url.to_string()),
            ImageSrc::Bytes | ImageSrc::Reader => None,
        }
    }

    pub fn source_path(&self) -> Option<PathBuf> {
        match &self.src {
            ImageSrc::File { path } => Some(path.to_owned()),
            _ => None,
        }
    }

    pub fn source_url(&self) -> Option<Url> {
        match &self.src {
            ImageSrc::Url { url } => Some(url.to_owned()),
            _ => None,
        }
    }
}
