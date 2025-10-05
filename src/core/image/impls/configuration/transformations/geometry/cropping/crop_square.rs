use crate::{
    image::{utils::to_nonzero_u32_with_context, TransformOp},
    Image,
};

impl Image {
    // takes the largest possible square from the center
    pub fn crop_square(&mut self) -> &mut Self {
        let (w, h) = (self.width(), self.height());
        let side = w.min(h);

        if w == h {
            return self;
        }

        let x0 = (w - side) / 2;
        let y0 = (h - side) / 2;

        self.config.pipeline.borrow_mut().push(TransformOp::Crop(x0, y0, side, side));

        let new_size = to_nonzero_u32_with_context(side, "Square crop");
        self.set_size(new_size, new_size);
        self
    }
}
