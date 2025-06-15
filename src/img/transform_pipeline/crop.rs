use crate::TransformPipeline;

impl TransformPipeline {
    pub fn crop(&mut self, x: u32, y: u32, w: u32, h: u32) -> &mut Self {
        self.crop_rect = Some(match self.crop_rect {
            // No crop yet → this becomes the first crop
            None => (x, y, w, h),

            // Crop at (x,y,w,h) *inside* previous crop.
            Some((ox, oy, ow, oh)) => {
                let nx = ox.saturating_add(x);
                let ny = oy.saturating_add(y);

                let nw = ow.saturating_sub(x).min(w);
                let nh = oh.saturating_sub(y).min(h);

                (nx, ny, nw, nh)
            }
        });

        self
    }
}
