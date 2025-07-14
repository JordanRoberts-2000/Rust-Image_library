use crate::{image::utils::clamp_ratio, CropEdge, Image, Result};

impl Image {
    pub async fn crop_ratio(&self, edge: CropEdge, ratio: f32) -> Result<&Self> {
        let original_ratio = ratio;
        let ratio = clamp_ratio(ratio);

        if (original_ratio - ratio).abs() > f32::EPSILON {
            log::warn!("Crop ratio {} was clamped to {}", original_ratio, ratio);
        }

        let (w, h) = {
            let state = self.state.read().await;
            (state.width.get(), state.height.get())
        };

        let drop_f = match edge {
            CropEdge::Left | CropEdge::Right | CropEdge::Horizontal => w as f32 * ratio,
            CropEdge::Top | CropEdge::Bottom | CropEdge::Vertical => h as f32 * ratio,
        }
        .round() as u32;

        self.crop_pixels(edge, drop_f).await
    }
}
