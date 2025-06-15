use crate::Img;

impl Img {
    pub fn crop_y_ratio(&mut self, ratio: f32) -> &mut Self {
        let ratio = Self::clamp_ratio(ratio);
        let (w, h) = (self.width, self.height);

        let total_drop = (h as f32 * ratio).round() as u32;
        let half = total_drop / 2;
        let new_h = h.saturating_sub(total_drop);

        self.transform_pipeline.crop(0, half, w, new_h);

        self.height = new_h;
        self.aspect_ratio = w as f32 / new_h as f32;

        self
    }
}
