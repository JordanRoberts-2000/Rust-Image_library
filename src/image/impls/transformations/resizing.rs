use crate::{Image, TransformOp};

impl Image {
    pub fn max_size(&mut self, size: u32) -> &mut Self {
        self.config.pipeline.push(TransformOp::MaxSize(size.max(1)));

        // Scale down proportionally if either dimension exceeds max_size
        let scale = (size as f32 / self.width.max(self.height) as f32).min(1.0);

        self.width = (self.width as f32 * scale) as u32;
        self.height = (self.height as f32 * scale) as u32;
        self.aspect_ratio = self.width as f32 / self.height as f32;

        self
    }

    pub fn resize(&mut self, width: u32, height: u32) -> &mut Self {
        self.config
            .pipeline
            .push(TransformOp::Resize(width.max(1), height.max(1)));

        // Proportional resize - fit within bounds while maintaining aspect ratio
        let width_scale = width as f32 / self.width as f32;
        let height_scale = height as f32 / self.height as f32;
        let scale = width_scale.min(height_scale);

        self.width = (self.width as f32 * scale) as u32;
        self.height = (self.height as f32 * scale) as u32;
        self.aspect_ratio = self.width as f32 / self.height as f32;

        self
    }

    pub fn resize_exact(&mut self, width: u32, height: u32) -> &mut Self {
        self.config
            .pipeline
            .push(TransformOp::ResizeExact(width.max(1), height.max(1)));

        self.width = width;
        self.height = height;
        self.aspect_ratio = width as f32 / height as f32;

        self
    }

    pub fn resize_fill(&mut self, width: u32, height: u32) -> &mut Self {
        self.config
            .pipeline
            .push(TransformOp::ResizeToFill(width.max(1), height.max(1)));

        // Scale to fill the entire target area (may crop)
        let width_scale = width as f32 / self.width as f32;
        let height_scale = height as f32 / self.height as f32;
        let scale = width_scale.max(height_scale);

        self.width = (self.width as f32 * scale) as u32;
        self.height = (self.height as f32 * scale) as u32;
        self.aspect_ratio = self.width as f32 / self.height as f32;

        self
    }
}
