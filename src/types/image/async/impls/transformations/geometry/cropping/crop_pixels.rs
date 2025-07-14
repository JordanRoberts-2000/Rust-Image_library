use crate::{
    image::{enums::TransformOp, utils::to_nonzero_u32_with_context},
    CropEdge, Image, Result,
};

impl Image {
    pub async fn crop_pixels(&self, edge: CropEdge, pixels: u32) -> Result<&Self> {
        let mut state = self.state.write().await;
        let (w, h) = (state.width.get(), state.height.get());

        let max_drop = match edge {
            CropEdge::Left | CropEdge::Right => w.saturating_sub(1),
            CropEdge::Top | CropEdge::Bottom => h.saturating_sub(1),
            CropEdge::Horizontal => (w.saturating_sub(1)) / 2 * 2,
            CropEdge::Vertical => (h.saturating_sub(1)) / 2 * 2,
        };

        let drop = pixels.min(max_drop);

        if drop < pixels {
            log::warn!(
                "Crop pixels {} exceeded maximum {} for edge {:?}, clamped to {}",
                pixels,
                max_drop,
                edge,
                drop
            );
        }

        if drop == 0 {
            return Ok(self); // no cropping applied
        }

        let (x, y, new_w, new_h) = match edge {
            CropEdge::Left => (drop, 0, w - drop, h),
            CropEdge::Right => (0, 0, w - drop, h),
            CropEdge::Top => (0, drop, w, h - drop),
            CropEdge::Bottom => (0, 0, w, h - drop),
            CropEdge::Horizontal => {
                let half = drop / 2;
                (half, 0, w - drop, h)
            }
            CropEdge::Vertical => {
                let half = drop / 2;
                (0, half, w, h - drop)
            }
        };

        state
            .config
            .pipeline
            .push(TransformOp::Crop(x, y, new_w, new_h));

        state.width = to_nonzero_u32_with_context(new_w, "Cropped width");
        state.height = to_nonzero_u32_with_context(new_h, "Cropped height");

        Ok(self)
    }
}
