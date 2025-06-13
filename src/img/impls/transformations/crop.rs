use crate::Img;

impl Img {
    pub fn crop_aspect(&mut self, aspect_ratio: f32) -> &mut Self {
        if (self.aspect_ratio - aspect_ratio).abs() < 0.01 {
            return self;
        }

        let (new_width, new_height) = if self.aspect_ratio > aspect_ratio {
            // Image is wider than target ratio - crop width
            let new_width = (self.height as f32 * aspect_ratio).round() as u32;
            (new_width, self.height)
        } else {
            // Image is taller than target ratio - crop height
            let new_height = (self.width as f32 / aspect_ratio).round() as u32;
            (self.width, new_height)
        };

        let x = (self.width - new_width) / 2;
        let y = (self.height - new_height) / 2;

        self.img = self.img.crop_imm(x, y, new_width, new_height);

        self.width = new_width;
        self.height = new_height;
        self.aspect_ratio;

        self
    }
}
