use crate::Img;

impl Img {
    pub fn crop_y(&mut self, pixels: u32) -> &mut Self {
        let (w, h) = (self.width, self.height);

        let max_drop = (h.saturating_sub(1)) / 2;
        let drop = pixels.min(max_drop);
        let new_h = h.saturating_sub(2 * drop);

        self.transform_pipeline.crop(0, drop, w, new_h);

        self.width = w;
        self.height = new_h;
        self.aspect_ratio = w as f32 / new_h as f32;

        self
    }
}
