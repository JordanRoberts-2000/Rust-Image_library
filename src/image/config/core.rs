#[derive(Default, Debug)]
pub struct ImageConfig {
    pub crop_rect: Option<(u32, u32, u32, u32)>,
    pub rotation_deg: u32,
    pub grayscale: bool,
    pub blur_intensity: Option<f32>,
    pub contrast: Option<f32>,
    pub resize_dimensions: Option<(u32, u32)>,
    pub resize_exact_dimensions: Option<(u32, u32)>,
    pub resize_fill_dimensions: Option<(u32, u32)>,
    pub max_size: Option<u32>,
}

impl ImageConfig {
    pub fn greyscale(&mut self) -> &mut Self {
        self.grayscale = true;
        self
    }

    pub fn blur(&mut self, intensity: f32) -> &mut Self {
        self.blur_intensity = Some(intensity);
        self
    }

    pub fn adjust_contrast(&mut self, c: f32) -> &mut Self {
        self.contrast = Some(c);
        self
    }

    pub fn resize(&mut self, w: u32, h: u32) -> &mut Self {
        self.resize_dimensions = Some((w, h));
        self
    }

    pub fn resize_exact(&mut self, w: u32, h: u32) -> &mut Self {
        self.resize_exact_dimensions = Some((w, h));
        self
    }

    pub fn resize_fill(&mut self, w: u32, h: u32) -> &mut Self {
        self.resize_fill_dimensions = Some((w, h));
        self
    }

    pub fn max_size(&mut self, max: u32) -> &mut Self {
        self.max_size = Some(max);
        self
    }
}
