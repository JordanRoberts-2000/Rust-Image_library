use crate::TransformPipeline;

impl TransformPipeline {
    pub fn crop(&mut self, x: u32, y: u32, w: u32, h: u32) -> &mut Self {
        self.crop_rect = Some(match self.crop_rect {
            None => (x, y, w, h),
            Some((ox, oy, ow, oh)) => {
                let nx = ox.saturating_add(x);
                let ny = oy.saturating_add(y);
                let nw = ow.saturating_sub(x).min(w);
                let nh = oh.saturating_sub(y).min(h);
                (nx, ny, nw.max(1), nh.max(1))
            }
        });
        self
    }
}
