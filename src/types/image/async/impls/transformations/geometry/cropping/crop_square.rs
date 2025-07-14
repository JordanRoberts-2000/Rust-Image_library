use crate::{
    image::{enums::TransformOp, utils::to_nonzero_u32_with_context},
    Image, Result,
};

impl Image {
    /// Takes the largest possible square from the center
    pub async fn crop_square(&self) -> Result<&Self> {
        let (x0, y0, side) = {
            let state = self.state.read().await;
            let (w, h) = (state.width.get(), state.height.get());

            if w == h {
                return Ok(self);
            }

            let side = w.min(h);
            let x0 = (w - side) / 2;
            let y0 = (h - side) / 2;
            (x0, y0, side)
        };

        let mut state = self.state.write().await;

        state
            .config
            .pipeline
            .push(TransformOp::Crop(x0, y0, side, side));
        state.width = to_nonzero_u32_with_context(side, "Square cropped width");
        state.height = to_nonzero_u32_with_context(side, "Square cropped height");

        Ok(self)
    }
}
