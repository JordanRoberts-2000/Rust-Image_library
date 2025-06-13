use crate::{Img, ImgSrc};

impl Img {
    pub fn describe_source(&self) -> String {
        match &self.src {
            ImgSrc::File { path } => format!("file “{}”", path.display()),
            ImgSrc::Url { url } => format!("URL “{}”", url),
            ImgSrc::Bytes => "raw bytes".into(),
            ImgSrc::Reader => "reader".into(),
        }
    }
}
