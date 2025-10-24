use crate::{utils::gcd, Image};

impl Image {
    pub fn width(&self) -> u32 {
        let decoded = self.decoded();
        decoded.dimensions().0
    }

    pub fn height(&self) -> u32 {
        let decoded = self.decoded();
        decoded.dimensions().1
    }

    pub fn dimensions(&self) -> (u32, u32) {
        let decoded = self.decoded();
        decoded.dimensions()
    }

    pub fn aspect_ratio(&self) -> f32 {
        let (w, h) = self.dimensions();
        w as f32 / h as f32
    }

    pub fn aspect_ratio_str(&self) -> String {
        let (w, h) = self.dimensions();
        let gcd = gcd(w, h);
        let x = w / gcd;
        let y = h / gcd;
        format!("{}:{}", x, y)
    }
}
