use image::imageops::FilterType;

use crate::Image;

impl Image {
    pub fn max_size(&mut self, max_size: u32) -> &mut Self {
        if self.width > max_size || self.height > max_size {
            self.raw = self.raw.thumbnail(max_size, max_size);
            self.width = self.raw.width();
            self.height = self.raw.height();
        }

        self
    }

    pub fn resize(&mut self, width: u32, height: u32) -> &mut Self {
        self.height = height;
        self.width = width;
        self.raw = self.raw.resize(width, height, FilterType::Lanczos3);

        self
    }

    pub fn resize_exact(&mut self, width: u32, height: u32) -> &mut Self {
        self.height = height;
        self.width = width;
        self.aspect_ratio = width as f32 / height as f32;
        self.raw = self.raw.resize_exact(width, height, FilterType::Lanczos3);

        self
    }
}
