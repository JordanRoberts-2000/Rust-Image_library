use crate::Img;

impl Img {
    pub fn clamp_ratio(ratio: f32) -> f32 {
        if !(0.0..=1.0).contains(&ratio) {
            log::warn!(
                "crop_bottom_ratio: {} is outside [0.0,1.0], clamping to range",
                ratio
            );
            ratio.clamp(0.0, 1.0)
        } else {
            ratio
        }
    }
}
