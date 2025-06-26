use crate::{CropEdge, Image, TransformOp};

impl Image {
    pub fn crop_pixels(&mut self, edge: CropEdge, pixels: u32) -> &mut Self {
        let (w, h) = (self.width, self.height);

        let drop = pixels.min(match edge {
            CropEdge::Left | CropEdge::Right => w.saturating_sub(1),
            CropEdge::Top | CropEdge::Bottom => h.saturating_sub(1),
            CropEdge::Horizontal => (w.saturating_sub(1)) / 2 * 2,
            CropEdge::Vertical => (h.saturating_sub(1)) / 2 * 2,
        });

        let (x, y, new_w, new_h) = match edge {
            CropEdge::Left => (drop, 0, w - drop, h),
            CropEdge::Right => (0, 0, w - drop, h),
            CropEdge::Top => (0, drop, w, h - drop),
            CropEdge::Bottom => (0, 0, w, h - drop),
            CropEdge::Horizontal => {
                let half = drop / 2;
                (half, 0, w - drop, h)
            }
            CropEdge::Vertical => {
                let half = drop / 2;
                (0, half, w, h - drop)
            }
        };

        self.config
            .pipeline
            .push(TransformOp::Crop(x, y, new_w, new_h));

        self.width = new_w;
        self.height = new_h;
        self.aspect_ratio = new_w as f32 / new_h as f32;

        self
    }
}
