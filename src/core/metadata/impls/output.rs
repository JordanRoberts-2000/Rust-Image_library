use crate::{metadata::utils::gcd, ImageMetadata};

impl ImageMetadata {
    pub fn width(&self) -> u32 {
        self.width.get()
    }

    pub fn height(&self) -> u32 {
        self.height.get()
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width(), self.height())
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.width() as f32 / self.height() as f32
    }

    pub fn aspect_ratio_str(&self) -> String {
        let gcd = gcd(self.width(), self.height());
        let x = self.width() / gcd;
        let y = self.height() / gcd;
        format!("{}:{}", x, y)
    }
}

#[cfg(test)]
mod tests {
    use {
        crate::{ImageFormat, ImageMetadata},
        std::num::NonZeroU32,
    };

    #[test]
    fn test_image_metadata_aspect_ratio_str() {
        let md = ImageMetadata {
            format: ImageFormat::Png,
            width: NonZeroU32::new(1920).unwrap(),
            height: NonZeroU32::new(1080).unwrap(),
        };
        assert_eq!(md.aspect_ratio_str(), "16:9");
    }
}
