use crate::Image;

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
}
