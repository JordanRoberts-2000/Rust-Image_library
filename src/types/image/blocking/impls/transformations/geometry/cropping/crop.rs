use crate::{
    blocking::Image,
    image::{enums::TransformOp, utils::to_nonzero_u32_with_context},
};

impl Image {
    pub fn crop(&mut self, x: u32, y: u32, w: u32, h: u32) -> &mut Self {
        let (orig_w, orig_h) = (self.width.get(), self.height.get());

        let original_x = x;
        let original_y = y;
        let original_w = w;
        let original_h = h;

        let x = x.min(orig_w.saturating_sub(1));
        let y = y.min(orig_h.saturating_sub(1));

        let w = w
            .min(orig_w.saturating_sub(x)) // can't be wider than "rest of row"
            .max(1); // but at least 1px
        let h = h
            .min(orig_h.saturating_sub(y)) // can't be taller than "rest of column"
            .max(1); // but at least 1px

        if x != original_x {
            log::warn!(
                "Crop x {} exceeded image width {}, clamped to {}",
                original_x,
                orig_w,
                x
            );
        }
        if y != original_y {
            log::warn!(
                "Crop y {} exceeded image height {}, clamped to {}",
                original_y,
                orig_h,
                y
            );
        }
        if w != original_w {
            log::warn!(
                "Crop width {} exceeded available space, clamped to {}",
                original_w,
                w
            );
        }
        if h != original_h {
            log::warn!(
                "Crop height {} exceeded available space, clamped to {}",
                original_h,
                h
            );
        }

        if x == 0 && y == 0 && w == orig_w && h == orig_h {
            return self;
        }

        self.config.pipeline.push(TransformOp::Crop(x, y, w, h));

        self.width = to_nonzero_u32_with_context(w, "Cropped width");
        self.height = to_nonzero_u32_with_context(h, "Cropped height");

        self
    }
}
