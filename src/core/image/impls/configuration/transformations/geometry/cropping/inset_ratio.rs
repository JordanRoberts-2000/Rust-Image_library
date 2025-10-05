use crate::{
    image::{
        utils::{clamp_ratio, to_nonzero_u32_with_context},
        TransformOp,
    },
    Image,
};

impl Image {
    pub fn inset_ratio(&mut self, ratio: f32) -> &mut Self {
        let original_ratio = ratio;
        let ratio = clamp_ratio(ratio);

        if (original_ratio - ratio).abs() > f32::EPSILON {
            log::warn!("Inset ratio {} was clamped to {}", original_ratio, ratio);
        }

        let (w, h) = (self.width(), self.height());

        let dx = ((w as f32 * ratio).round() as u32).min(w.saturating_sub(1)) / 2;
        let dy = ((h as f32 * ratio).round() as u32).min(h.saturating_sub(1)) / 2;
        let new_w = w.saturating_sub(2 * dx);
        let new_h = h.saturating_sub(2 * dy);

        if dx == 0 && dy == 0 {
            return self;
        }

        self.config.pipeline.borrow_mut().push(TransformOp::Crop(dx, dy, new_w, new_h));

        let new_width = to_nonzero_u32_with_context(new_w, "Inset width");
        let new_height = to_nonzero_u32_with_context(new_h, "Inset height");
        self.set_size(new_width, new_height);

        self
    }
}
