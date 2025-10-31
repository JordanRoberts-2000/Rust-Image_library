use {crate::Image, std::path::Path};

impl Image {
    pub fn set_file_name(&mut self, file_name: impl AsRef<str>) -> &mut Self {
        self.config.file_name = file_name.as_ref().to_string();
        self
    }

    pub fn set_output_dir(&mut self, dir: impl AsRef<Path>) -> &mut Self {
        self.config.output_dir = dir.as_ref().to_path_buf();
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
}
