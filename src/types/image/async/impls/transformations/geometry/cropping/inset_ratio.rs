use crate::{
    image::{
        enums::TransformOp,
        utils::{clamp_ratio, to_nonzero_u32_with_context},
    },
    Image, Result,
};

impl Image {
    pub async fn inset_ratio(&self, ratio: f32) -> Result<&Self> {
        let original_ratio = ratio;
        let ratio = clamp_ratio(ratio);

        if (original_ratio - ratio).abs() > f32::EPSILON {
            log::warn!("Inset ratio {} was clamped to {}", original_ratio, ratio);
        }

        let mut state = self.state.write().await;

        let (w, h) = (state.width.get(), state.height.get());

        let dx = ((w as f32 * ratio).round() as u32).min(w.saturating_sub(1)) / 2;
        let dy = ((h as f32 * ratio).round() as u32).min(h.saturating_sub(1)) / 2;
        let new_w = w.saturating_sub(2 * dx);
        let new_h = h.saturating_sub(2 * dy);

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
