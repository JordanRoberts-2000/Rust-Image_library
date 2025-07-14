use crate::{
    image::{enums::TransformOp, r#async::ImageState, utils::to_nonzero_u32_with_context},
    Image, Result,
};

impl Image {
    pub async fn max_size(&self, size: u32) -> Result<&Self> {
        let size = to_nonzero_u32_with_context(size, "Max size");

        let mut state = self.state.write().await;
        let current_max = state.width.get().max(state.height.get());

        if current_max <= size.get() {
            return Ok(self);
        }

        state.config.pipeline.push(TransformOp::MaxSize(size));

        let scale = size.get() as f32 / current_max as f32;
        Self::apply_scale_internal(&mut state, scale);

        Ok(self)
    }

    pub async fn resize(&self, width: u32, height: u32) -> Result<&Self> {
        let width = to_nonzero_u32_with_context(width, "Resize width");
        let height = to_nonzero_u32_with_context(height, "Resize height");

        let mut state = self.state.write().await;

        let width_scale = width.get() as f32 / state.width.get() as f32;
        let height_scale = height.get() as f32 / state.height.get() as f32;
        let scale = width_scale.min(height_scale);

        if (scale - 1.0).abs() < f32::EPSILON {
            return Ok(self);
        }

        state
            .config
            .pipeline
            .push(TransformOp::Resize(width, height));
        Self::apply_scale_internal(&mut state, scale);

        Ok(self)
    }

    pub async fn resize_exact(&self, width: u32, height: u32) -> Result<&Self> {
        let width = to_nonzero_u32_with_context(width, "Resize exact width");
        let height = to_nonzero_u32_with_context(height, "Resize exact height");

        let mut state = self.state.write().await;

        if state.width == width && state.height == height {
            return Ok(self);
        }

        state
            .config
            .pipeline
            .push(TransformOp::ResizeExact(width, height));

        state.width = width;
        state.height = height;

        Ok(self)
    }

    pub async fn resize_fill(&self, width: u32, height: u32) -> Result<&Self> {
        let width = to_nonzero_u32_with_context(width, "Resize fill width");
        let height = to_nonzero_u32_with_context(height, "Resize fill height");

        let mut state = self.state.write().await;

        let width_scale = width.get() as f32 / state.width.get() as f32;
        let height_scale = height.get() as f32 / state.height.get() as f32;
        let scale = width_scale.max(height_scale);

        if (scale - 1.0).abs() < f32::EPSILON {
            return Ok(self);
        }

        state
            .config
            .pipeline
            .push(TransformOp::ResizeToFill(width, height));

        Self::apply_scale_internal(&mut state, scale);

        Ok(self)
    }

    fn apply_scale_internal(state: &mut ImageState, scale: f32) {
        let new_width = (state.width.get() as f32 * scale) as u32;
        let new_height = (state.height.get() as f32 * scale) as u32;

        state.width = to_nonzero_u32_with_context(new_width, "Scaled width");
        state.height = to_nonzero_u32_with_context(new_height, "Scaled height");
    }
}
