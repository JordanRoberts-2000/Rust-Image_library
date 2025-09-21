use {crate::Image, image::DynamicImage};

impl Image {
    pub(super) fn apply_transformations(&self, img: &mut DynamicImage) {
        let mut ops = self.config.pipeline.borrow_mut();
        if !ops.is_empty() {
            for op in ops.drain(..) {
                op.apply(img);
            }
        }
    }
}
