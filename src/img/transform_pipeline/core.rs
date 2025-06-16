use crate::ImageFormat;

#[derive(Default, Debug)]
pub struct TransformPipeline {
    pub(super) crop_rect: Option<(u32, u32, u32, u32)>,
    pub(super) rotation_deg: u32,
    pub(super) target_dimentions: Option<(u32, u32)>,
    pub(super) grayscale: bool,
    pub(super) blur_intensity: Option<f32>,
    pub(super) contrast: Option<f32>,
    pub(super) format: Option<ImageFormat>,
}

impl TransformPipeline {
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

    pub fn format(&mut self, fmt: ImageFormat) -> &mut Self {
        self.format = Some(fmt);
        self
    }
}
