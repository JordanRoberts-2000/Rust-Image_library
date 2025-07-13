use crate::{
    blocking::Image,
    image::{enums::TransformOp, utils::to_nonzero_u32_with_context},
};

impl Image {
    pub fn crop_aspect(&mut self, ratio: f32) -> &mut Self {
        let original_ratio = ratio;
        let ratio = ratio.max(0.001).min(1000.0);

        // Warn if ratio was clamped
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

        let (w, h) = (self.width.get(), self.height.get());
        let current = w as f32 / h as f32;

        if (current - ratio).abs() < f32::EPSILON {
            return self;
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

        self.config
            .pipeline
            .push(TransformOp::Crop(x0, y0, new_w, new_h));

        self.width = to_nonzero_u32_with_context(new_w, "Cropped width");
        self.height = to_nonzero_u32_with_context(new_h, "Cropped height");

        self
    }
}
