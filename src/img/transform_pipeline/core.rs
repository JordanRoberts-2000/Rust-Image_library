use crate::ImageFormat;

#[derive(Default, Debug)]
pub struct TransformPipeline {
    pub(super) crop_rect: Option<(u32, u32, u32, u32)>,
    pub(super) rotation_deg: u32,
    pub(super) target_dimentions: Option<(u32, u32)>,
    pub(super) grayscale: bool,
    pub(super) blur_intensity: Option<f32>,
    pub(super) format: Option<ImageFormat>,
}

impl TransformPipeline {
    pub fn greyscale(mut self) -> Self {
        self.grayscale = true;
        self
    }

    pub fn blur(mut self, intensity: f32) -> Self {
        self.blur_intensity = Some(intensity);
        self
    }

    pub fn format(mut self, fmt: ImageFormat) -> Self {
        self.format = Some(fmt);
        self
    }
}
