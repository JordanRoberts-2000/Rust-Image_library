use crate::Image;

impl Image {
    pub fn file_name(&mut self, file_name: impl AsRef<str>) -> &mut Self {
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

    pub fn build_file_name(&self) -> String {
        format!(
            "{}{}{}",
            self.config.prefix.as_deref().unwrap_or(""),
            self.config.file_name,
            self.config.suffix.as_deref().unwrap_or("")
        )
    }
}
