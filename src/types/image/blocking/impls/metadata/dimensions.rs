use crate::{blocking::Image, image::utils::greatest_common_divisor};

impl Image {
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
        self.width.get() as f32 / self.height.get() as f32
    }

    pub fn aspect_ratio_str(&self) -> String {
        let gcd = greatest_common_divisor(self.width(), self.height());
        let x = self.width() / gcd;
        let y = self.height() / gcd;
        format!("{}:{}", x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn aspect_ratio_str_standalone(width: u32, height: u32) -> String {
        let gcd = greatest_common_divisor(width, height);
        let x = width / gcd;
        let y = height / gcd;
        format!("{}:{}", x, y)
    }

    #[test]
    fn test_standalone_aspect_ratio() {
        assert_eq!(aspect_ratio_str_standalone(1920, 1080), "16:9");
        assert_eq!(aspect_ratio_str_standalone(800, 600), "4:3");
        assert_eq!(aspect_ratio_str_standalone(1000, 1000), "1:1");
    }
}
