use crate::Image;

impl Image {
    pub fn inset_ratio(&mut self, ratio: f32) -> &mut Self {
        let ratio = Self::clamp_ratio(ratio);
        let (w, h) = (self.width, self.height);

        let dx = ((w as f32 * ratio).round() as u32).min(w.saturating_sub(1)) / 2;
        let dy = ((h as f32 * ratio).round() as u32).min(h.saturating_sub(1)) / 2;
        let new_w = w.saturating_sub(2 * dx);
        let new_h = h.saturating_sub(2 * dy);

        self.config.crop(dx, dy, new_w, new_h);

        self.width = new_w;
        self.height = new_h;
        self.aspect_ratio = new_w as f32 / new_h as f32;

        self
    }
}
