use crate::{enums::ImgSrc, Img};

impl Img {
    pub fn id(&self) -> String {
        match &self.src {
            ImgSrc::Local { path, .. } => path
                .file_name()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_else(|| path.to_string_lossy().into_owned()),
            ImgSrc::Remote { url, .. } => url.to_string(),
            ImgSrc::Bytes { id, .. } => id.to_owned(),
        }
    }
}
