use crate::Image;

impl Image {
    pub fn file_name(&self) -> String {
        format!(
            "{}{}{}.{}",
            self.config.prefix.as_deref().unwrap_or(""),
            self.config.file_name,
            self.config.suffix.as_deref().unwrap_or(""),
            self.encoding_format().primary_extension()
        )
    }

    pub fn extension(&self) -> &str {
        self.encoding_format().primary_extension()
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
