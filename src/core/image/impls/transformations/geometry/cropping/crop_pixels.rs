use crate::{
    image::{utils::to_nonzero_u32_with_context, TransformOp},
    CropEdge, Image,
};

impl Image {
    pub fn crop_pixels(&mut self, edge: CropEdge, pixels: u32) -> &mut Self {
        let (w, h) = (self.width(), self.height());

        let max_drop = match edge {
            CropEdge::Left | CropEdge::Right => w.saturating_sub(1),
            CropEdge::Top | CropEdge::Bottom => h.saturating_sub(1),
            CropEdge::Horizontal => (w.saturating_sub(1)) / 2 * 2,
            CropEdge::Vertical => (h.saturating_sub(1)) / 2 * 2,
        };

        let drop = pixels.min(max_drop);

        if drop < pixels {
            log::warn!(
                "Crop pixels {} exceeded maximum {} for edge {:?}, clamped to {}",
                pixels,
                max_drop,
                edge,
                drop
            );
        }

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

        if drop == 0 {
            return self;
        }

        self.config.pipeline.borrow_mut().push(TransformOp::Crop(x, y, new_w, new_h));

        let new_width = to_nonzero_u32_with_context(new_w, "Cropped width");
        let new_height = to_nonzero_u32_with_context(new_h, "Cropped height");

        self.set_size(new_width, new_height);

        self
    }
}
