use crate::{blocking::Image, image::utils::clamp_ratio, CropEdge};

impl Image {
    pub fn crop_ratio(&mut self, edge: CropEdge, ratio: f32) -> &mut Self {
        let original_ratio = ratio;
        let ratio = clamp_ratio(ratio);

        // Warn if ratio was clamped
        if (original_ratio - ratio).abs() > f32::EPSILON {
            log::warn!("Crop ratio {} was clamped to {}", original_ratio, ratio);
        }

        let (w, h) = (self.width.get(), self.height.get());

        let drop_f = match edge {
            CropEdge::Left | CropEdge::Right | CropEdge::Horizontal => w as f32 * ratio,
            CropEdge::Top | CropEdge::Bottom | CropEdge::Vertical => h as f32 * ratio,
        }
        .round() as u32;

        self.crop_pixels(edge, drop_f)
    }
}
