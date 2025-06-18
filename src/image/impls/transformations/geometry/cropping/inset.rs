use crate::Image;

impl Image {
    pub fn inset(&mut self, pixels: u32) -> &mut Self {
        let (w, h) = (self.width, self.height);

        // clamp so we never invert and always leave ≥1px
        let dx = pixels.min((w.saturating_sub(1)) / 2);
        let dy = pixels.min((h.saturating_sub(1)) / 2);

        let new_w = w - 2 * dx;
        let new_h = h - 2 * dy;

        self.config.crop(dx, dy, new_w, new_h);

        self.width = new_w;
        self.height = new_h;
        self.aspect_ratio = new_w as f32 / new_h as f32;

        self
    }
}
