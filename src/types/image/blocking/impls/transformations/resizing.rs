use crate::{
    blocking::Image,
    image::{enums::TransformOp, utils::to_nonzero_u32_with_context},
};

impl Image {
    pub fn max_size(&mut self, size: u32) -> &mut Self {
        let size = to_nonzero_u32_with_context(size, "Max size");

        let current_max = self.width().max(self.height.get());

        if current_max <= size.get() {
            return self;
        }

        self.config.pipeline.push(TransformOp::MaxSize(size));

        let scale = size.get() as f32 / current_max as f32;
        self.apply_scale(scale);

        self
    }

    pub fn resize(&mut self, width: u32, height: u32) -> &mut Self {
        let width = to_nonzero_u32_with_context(width, "Resize width");
        let height = to_nonzero_u32_with_context(height, "Resize height");

        // Calculate proportional scale - fit within bounds while maintaining aspect ratio
        let width_scale = width.get() as f32 / self.width.get() as f32;
        let height_scale = height.get() as f32 / self.height.get() as f32;
        let scale = width_scale.min(height_scale);

        if (scale - 1.0).abs() < f32::EPSILON {
            return self;
        }

        self.config.pipeline.push(TransformOp::Resize(width, height));
        self.apply_scale(scale);

        self
    }

    pub fn resize_exact(&mut self, width: u32, height: u32) -> &mut Self {
        let width = to_nonzero_u32_with_context(width, "Resize exact width");
        let height = to_nonzero_u32_with_context(height, "Resize exact height");

        if self.width == width && self.height == height {
            return self;
        }

        self.config.pipeline.push(TransformOp::ResizeExact(width, height));

        self.width = width;
        self.height = height;

        self
    }

    pub fn resize_fill(&mut self, width: u32, height: u32) -> &mut Self {
        let width = to_nonzero_u32_with_context(width, "Resize fill width");
        let height = to_nonzero_u32_with_context(height, "Resize fill height");

        // Scale to fill the entire target area (may crop)
        let width_scale = width.get() as f32 / self.width.get() as f32;
        let height_scale = height.get() as f32 / self.height.get() as f32;
        let scale = width_scale.max(height_scale);

        if (scale - 1.0).abs() < f32::EPSILON {
            return self;
        }

        self.config.pipeline.push(TransformOp::ResizeToFill(width, height));
        self.apply_scale(scale);

        self
    }

    fn apply_scale(&mut self, scale: f32) {
        let new_width = (self.width.get() as f32 * scale) as u32;
        let new_height = (self.height.get() as f32 * scale) as u32;

        self.width = to_nonzero_u32_with_context(new_width, "Scaled width");
        self.height = to_nonzero_u32_with_context(new_height, "Scaled height");
    }
}
