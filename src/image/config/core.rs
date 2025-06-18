#[derive(Debug)]
pub enum Transform {
    Crop(u32, u32, u32, u32),
    Rotate(u32),
    Resize(u32, u32),
    ResizeExact(u32, u32),
    ResizeToFill(u32, u32),
    MaxSize(u32),
    Grayscale,
    Contrast(f32),
    Blur(f32),
}

#[derive(Default, Debug)]
pub struct ImageConfig {
    pub pipeline: Vec<Transform>,
}

impl ImageConfig {
    pub fn crop(&mut self, x: u32, y: u32, w: u32, h: u32) -> &mut Self {
        self.pipeline.push(Transform::Crop(x, y, w, h));
        self
    }

    pub fn rotate_90(&mut self) -> &mut Self {
        self.pipeline.push(Transform::Rotate(90));
        self
    }

    pub fn rotate_180(&mut self) -> &mut Self {
        self.pipeline.push(Transform::Rotate(180));
        self
    }

    pub fn rotate_270(&mut self) -> &mut Self {
        self.pipeline.push(Transform::Rotate(270));
        self
    }

    pub fn resize(&mut self, w: u32, h: u32) -> &mut Self {
        self.pipeline.push(Transform::Resize(w.max(1), h.max(1)));
        self
    }

    pub fn resize_exact(&mut self, w: u32, h: u32) -> &mut Self {
        self.pipeline
            .push(Transform::ResizeExact(w.max(1), h.max(1)));
        self
    }

    pub fn resize_fill(&mut self, w: u32, h: u32) -> &mut Self {
        self.pipeline
            .push(Transform::ResizeToFill(w.max(1), h.max(1)));
        self
    }

    pub fn max_size(&mut self, max: u32) -> &mut Self {
        self.pipeline.push(Transform::MaxSize(max.max(1)));
        self
    }

    pub fn grayscale(&mut self) -> &mut Self {
        self.pipeline.push(Transform::Grayscale);
        self
    }

    pub fn adjust_contrast(&mut self, c: f32) -> &mut Self {
        self.pipeline.push(Transform::Contrast(c));
        self
    }

    pub fn blur(&mut self, sigma: f32) -> &mut Self {
        self.pipeline.push(Transform::Blur(sigma));
        self
    }
}
