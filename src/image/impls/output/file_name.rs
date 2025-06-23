use crate::Image;

impl Image {
    pub fn file_name(&mut self, file_name: impl AsRef<str>) -> &mut Self {
        self.config.file_name = file_name.as_ref().to_string();
        self
    }
}
