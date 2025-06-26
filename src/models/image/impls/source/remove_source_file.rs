use crate::Image;

impl Image {
    pub fn remove_source_file(&mut self) -> &mut Self {
        self.config.remove_source = true;
        self
    }
}
