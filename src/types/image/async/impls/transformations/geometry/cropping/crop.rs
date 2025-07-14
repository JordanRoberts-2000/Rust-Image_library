use crate::{
    image::{enums::TransformOp, utils::to_nonzero_u32_with_context},
    Image, Result,
};

impl Image {
    pub async fn crop(&self, x: u32, y: u32, w: u32, h: u32) -> Result<&Self> {
        let (orig_w, orig_h) = {
            let state = self.state.read().await;
            (state.width.get(), state.height.get())
        };

        let original_x = x;
        let original_y = y;
        let original_w = w;
        let original_h = h;

        let x = x.min(orig_w.saturating_sub(1));
        let y = y.min(orig_h.saturating_sub(1));

        let w = w.min(orig_w.saturating_sub(x)).max(1);
        let h = h.min(orig_h.saturating_sub(y)).max(1);

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
            return Ok(self);
        }

        let mut state = self.state.write().await;
        state.config.pipeline.push(TransformOp::Crop(x, y, w, h));
        state.width = to_nonzero_u32_with_context(w, "Cropped width");
        state.height = to_nonzero_u32_with_context(h, "Cropped height");

        Ok(self)
    }
}
