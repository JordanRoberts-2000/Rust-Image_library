use crate::{Image, ImageSrc};

impl Image {
    pub fn describe_source(&self) -> String {
        match &self.src {
            ImageSrc::File(path) => format!("file “{}”", path.display()),
            ImageSrc::Url(url) => format!("URL “{}”", url),
            ImageSrc::Bytes => "raw bytes".into(),
            ImageSrc::Reader => "reader".into(),
        }
    }
}
