use crate::{
    image::{enums::TransformOp, utils::to_nonzero_u32_with_context},
    Image, Result,
};

impl Image {
    pub async fn inset(&self, pixels: u32) -> Result<&Self> {
        let mut state = self.state.write().await;
        let (w, h) = (state.width.get(), state.height.get());

        // Clamp so we never invert and always leave ≥1px
        let max_dx = (w.saturating_sub(1)) / 2;
        let max_dy = (h.saturating_sub(1)) / 2;

        let dx = pixels.min(max_dx);
        let dy = pixels.min(max_dy);

        if dx < pixels {
            log::warn!(
                "Inset pixels {} exceeded maximum horizontal inset {}, clamped to {}",
                pixels,
                max_dx,
                dx
            );
        }
        if dy < pixels {
            log::warn!(
                "Inset pixels {} exceeded maximum vertical inset {}, clamped to {}",
                pixels,
                max_dy,
                dy
            );
        }

        let new_w = w - 2 * dx;
        let new_h = h - 2 * dy;

        if dx == 0 && dy == 0 {
            return Ok(self);
        }

        state
            .config
            .pipeline
            .push(TransformOp::Crop(dx, dy, new_w, new_h));

        state.width = to_nonzero_u32_with_context(new_w, "Inset width");
        state.height = to_nonzero_u32_with_context(new_h, "Inset height");

        Ok(self)
    }
}
