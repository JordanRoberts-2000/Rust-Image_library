use crate::Img;

impl Img {
    pub fn crop_left(&mut self, n: u32) -> &mut Self {
        let (w, h) = (self.width, self.height);

        let drop = n.min(w.saturating_sub(1));
        let new_w = w - drop;

        self.transform_pipeline.crop(n, 0, new_w, h);
        self.width = new_w;
        self.height = h;
        self.aspect_ratio = new_w as f32 / h as f32;

        self
    }
}
