use image::imageops::FilterType;

use crate::Img;

impl Img {
    pub fn max_size(&mut self, max_size: u32) -> &mut Self {
        if self.width > max_size || self.height > max_size {
            self.img = self.img.thumbnail(max_size, max_size);
            self.width = self.img.width();
            self.height = self.img.height();
        }

        self
    }

    pub fn resize(&mut self, width: u32, height: u32) -> &mut Self {
        self.height = height;
        self.width = width;
        self.img = self.img.resize(width, height, FilterType::Lanczos3);

        self
    }

    pub fn resize_exact(&mut self, width: u32, height: u32) -> &mut Self {
        self.height = height;
        self.width = width;
        self.aspect_ratio = width as f32 / height as f32;
        self.img = self.img.resize_exact(width, height, FilterType::Lanczos3);

        self
    }
}
