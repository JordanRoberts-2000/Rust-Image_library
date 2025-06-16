use crate::Img;

impl Img {
    // takes the largest possible square from the center
    pub fn crop_square(&mut self) -> &mut Self {
        let (w, h) = (self.width, self.height);
        let side = w.min(h);

        let x0 = (w - side) / 2;
        let y0 = (h - side) / 2;

        self.transform_pipeline.crop(x0, y0, side, side);

        self.width = side;
        self.height = side;
        self.aspect_ratio = side as f32 / side as f32;

        self
    }
}
