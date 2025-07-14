use crate::{
    image::{enums::TransformOp, utils::to_nonzero_u32_with_context},
    Image, Result,
};

impl Image {
    pub async fn crop_aspect(&self, ratio: f32) -> Result<&Self> {
        let original_ratio = ratio;
        let ratio = ratio.clamp(0.001, 1000.0);

        if (original_ratio - ratio).abs() > f32::EPSILON {
            if original_ratio < 0.001 {
                log::warn!(
                    "Crop aspect ratio {} too small, clamped to 0.001",
                    original_ratio
                );
            } else if original_ratio > 1000.0 {
                log::warn!(
                    "Crop aspect ratio {} too large, clamped to 1000.0",
                    original_ratio
                );
            }
        }

        let mut state = self.state.write().await;

        let (w, h) = (state.width.get(), state.height.get());
        let current = w as f32 / h as f32;

        if (current - ratio).abs() < f32::EPSILON {
            return Ok(self);
        }

        let (new_w, new_h) = if current > ratio {
            let nw = ((h as f32 * ratio).round() as u32).max(1);
            (nw, h)
        } else {
            let nh = ((w as f32 / ratio).round() as u32).max(1);
            (w, nh)
        };

        let x0 = (w - new_w) / 2;
        let y0 = (h - new_h) / 2;

        state
            .config
            .pipeline
            .push(TransformOp::Crop(x0, y0, new_w, new_h));

        state.width = to_nonzero_u32_with_context(new_w, "Cropped width");
        state.height = to_nonzero_u32_with_context(new_h, "Cropped height");

        Ok(self)
    }
}
