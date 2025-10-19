use crate::{
    image::{utils::to_nonzero_u32_with_context, TransformOp},
    Image,
};

impl Image {
    pub fn max_size(&mut self, size: u32) -> &mut Self {
        let size = to_nonzero_u32_with_context(size, "Max size");

        self.config.pipeline.borrow_mut().push(TransformOp::MaxSize(size));
        self
    }

    pub fn resize(&mut self, width: u32, height: u32) -> &mut Self {
        let width = to_nonzero_u32_with_context(width, "Resize width");
        let height = to_nonzero_u32_with_context(height, "Resize height");

        self.config.pipeline.borrow_mut().push(TransformOp::Resize(width, height));
        self
    }

    pub fn resize_exact(&mut self, width: u32, height: u32) -> &mut Self {
        let width = to_nonzero_u32_with_context(width, "Resize exact width");
        let height = to_nonzero_u32_with_context(height, "Resize exact height");

        self.config.pipeline.borrow_mut().push(TransformOp::ResizeExact(width, height));
        self
    }

    pub fn resize_fill(&mut self, width: u32, height: u32) -> &mut Self {
        let width = to_nonzero_u32_with_context(width, "Resize fill width");
        let height = to_nonzero_u32_with_context(height, "Resize fill height");

        self.config.pipeline.borrow_mut().push(TransformOp::ResizeToFill(width, height));
        self
    }
}
