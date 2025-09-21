use {
    crate::{image::ImageSrc, Image},
    std::path::PathBuf,
    url::Url,
};

impl Image {
    pub fn with_file_name(&mut self, file_name: impl AsRef<str>) -> &mut Self {
        self.config.file_name = file_name.as_ref().to_string();
        self
    }

    pub fn prefix(&mut self, prefix: impl AsRef<str>) -> &mut Self {
        self.config.prefix = Some(prefix.as_ref().to_string());
        self
    }

    pub fn suffix(&mut self, suffix: impl AsRef<str>) -> &mut Self {
        self.config.suffix = Some(suffix.as_ref().to_string());
        self
    }

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
}
