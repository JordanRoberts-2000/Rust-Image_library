use crate::{Image, ImageSrc};

impl Image {
    pub(crate) fn error_src(&self) -> Option<&ImageSrc> {
        Some(&self.src)
    }
}
