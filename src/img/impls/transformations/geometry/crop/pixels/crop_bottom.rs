use crate::Img;

impl Img {
    pub fn crop_bottom(&mut self, n: u32) -> &mut Self {
        let (w, h) = (self.width, self.height);

        let drop = n.min(h.saturating_sub(1));
        let new_h = h - drop;

        self.transform_pipeline.crop(0, 0, w, new_h);

        self.width = w;
        self.height = new_h;
        self.aspect_ratio = w as f32 / new_h as f32;

        self
    }
}
