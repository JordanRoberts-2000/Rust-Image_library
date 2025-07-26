use crate::{
    blocking::Image,
    image::{enums::TransformOp, utils::to_nonzero_u32_with_context},
};

impl Image {
    // takes the largest possible square from the center
    pub fn crop_square(&mut self) -> &mut Self {
        let (w, h) = (self.width.get(), self.height.get());
        let side = w.min(h);

        if w == h {
            return self;
        }

        let x0 = (w - side) / 2;
        let y0 = (h - side) / 2;

        self.config.pipeline.push(TransformOp::Crop(x0, y0, side, side));

        self.width = to_nonzero_u32_with_context(side, "Square cropped width");
        self.height = to_nonzero_u32_with_context(side, "Square cropped height");

        self
    }
}
