use crate::Img;

impl Img {
    pub fn crop_x_ratio(&mut self, ratio: f32) -> &mut Self {
        let ratio = Self::clamp_ratio(ratio);
        let (w, h) = (self.width, self.height);

        let total_drop = (w as f32 * ratio).round() as u32;
        let half = total_drop / 2;
        let new_w = w.saturating_sub(total_drop);

        self.transform_pipeline.crop(half, 0, new_w, h);

        self.width = new_w;
        self.aspect_ratio = new_w as f32 / h as f32;

        self
    }
}
