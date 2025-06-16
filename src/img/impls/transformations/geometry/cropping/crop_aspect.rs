use crate::Img;

impl Img {
    pub fn crop_aspect(&mut self, ratio: f32) -> &mut Self {
        let ratio = ratio.max(0.001).min(1000.0);

        let (w, h) = (self.width, self.height);
        let current = w as f32 / h as f32;

        let (new_w, new_h) = if current > ratio {
            let nw = ((h as f32 * ratio).round() as u32).max(1);
            (nw, h)
        } else {
            let nh = ((w as f32 / ratio).round() as u32).max(1);
            (w, nh)
        };

        let x0 = (w - new_w) / 2;
        let y0 = (h - new_h) / 2;

        self.transform_pipeline.crop(x0, y0, new_w, new_h);

        self.width = new_w;
        self.height = new_h;
        self.aspect_ratio = new_w as f32 / new_h as f32;

        self
    }
}
