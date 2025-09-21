use {
    crate::{Image, ImageSrc},
    std::path::PathBuf,
    url::Url,
};

impl Image {
    pub fn file_name(&self) -> String {
        format!(
            "{}{}{}.{}",
            self.config.prefix.as_deref().unwrap_or(""),
            self.config.file_name,
            self.config.suffix.as_deref().unwrap_or(""),
            self.metadata.format.extension()
        )
    }

    pub fn extension(&self) -> &str {
        self.metadata.format.extension()
    }

    pub fn file_stem(&self) -> String {
        format!(
            "{}{}{}",
            self.config.prefix.as_deref().unwrap_or(""),
            self.config.file_name,
            self.config.suffix.as_deref().unwrap_or("")
        )
    }

    pub fn src(&self) -> &ImageSrc {
        &self.src
    }

    pub fn source_path(&self) -> Option<PathBuf> {
        match &self.src {
            ImageSrc::File(path) => Some(path.to_owned()),
            _ => None,
        }
    }

    pub fn source_url(&self) -> Option<Url> {
        match &self.src {
            ImageSrc::Url(url) => Some(url.to_owned()),
            _ => None,
        }
    }

    pub(crate) fn error_src(&self) -> Option<&ImageSrc> {
        Some(&self.src)
    }
}
