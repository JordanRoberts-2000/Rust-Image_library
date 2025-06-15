use crate::Img;

impl Img {
    pub fn crop_right_ratio(&mut self, ratio: f32) -> &mut Self {
        let ratio = Self::clamp_ratio(ratio);
        let (w, h) = (self.width, self.height);

        let raw_drop = (w as f32 * ratio).round() as u32;
        let drop = raw_drop.min(w.saturating_sub(1));
        let new_w = w.saturating_sub(drop);

        self.transform_pipeline.crop(0, 0, new_w, h);

        self.width = new_w;
        self.aspect_ratio = new_w as f32 / h as f32;

        self
    }
}
