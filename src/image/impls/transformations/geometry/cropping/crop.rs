use crate::Image;

impl Image {
    pub fn crop(&mut self, x: u32, y: u32, w: u32, h: u32) -> &mut Self {
        let (orig_w, orig_h) = (self.width, self.height);

        let x = x.min(orig_w.saturating_sub(1));
        let y = y.min(orig_h.saturating_sub(1));

        let w = w
            .min(orig_w.saturating_sub(x)) // can’t be wider than “rest of row”
            .max(1); // but at least 1px
        let h = h
            .min(orig_h.saturating_sub(y)) // can’t be taller than “rest of column”
            .max(1); // but at least 1px

        self.config.crop(x, y, w, h);

        self.width = w;
        self.height = h;
        self.aspect_ratio = w as f32 / h as f32;

        self
    }
}
