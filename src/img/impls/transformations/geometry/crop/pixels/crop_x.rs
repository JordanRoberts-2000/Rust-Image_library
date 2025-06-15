use crate::Img;

impl Img {
    pub fn crop_x(&mut self, pixels: u32) -> &mut Self {
        let (w, h) = (self.width, self.height);

        let max_drop = (w.saturating_sub(1)) / 2;
        let drop = pixels.min(max_drop);
        let new_w = w.saturating_sub(2 * drop);

        self.transform_pipeline.crop(drop, 0, new_w, h);

        self.width = new_w;
        self.height = h;
        self.aspect_ratio = new_w as f32 / h as f32;

        self
    }
}
