use crate::Img;

impl Img {
    pub fn crop_bottom_ratio(&mut self, ratio: f32) -> &mut Self {
        let ratio = Self::clamp_ratio(ratio);
        let (w, h) = (self.width, self.height);

        let raw_drop = (h as f32 * ratio).round() as u32;
        let drop = raw_drop.min(h.saturating_sub(1));
        let new_h = h.saturating_sub(drop);

        self.transform_pipeline.crop(0, 0, w, new_h);

        self.height = new_h;
        self.aspect_ratio = w as f32 / new_h as f32;

        self
    }
}
