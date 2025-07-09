use crate::{CropEdge, Image};

impl Image {
    pub fn crop_ratio(&mut self, edge: CropEdge, ratio: f32) -> &mut Self {
        let ratio = Self::clamp_ratio(ratio);
        let (w, h) = (self.width, self.height);

        let drop_f = match edge {
            CropEdge::Left | CropEdge::Right | CropEdge::Horizontal => w as f32 * ratio,
            CropEdge::Top | CropEdge::Bottom | CropEdge::Vertical => h as f32 * ratio,
        }
        .round() as u32;

        self.crop_pixels(edge, drop_f)
    }
}
