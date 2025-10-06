use {
    crate::{Image, ImageSrc},
    image::DynamicImage,
    std::cell::Ref,
};

impl Image {
    pub(crate) fn src(&self) -> ImageSrc {
        self.src.clone()
    }

    pub(crate) fn processed_image(&self) -> Ref<DynamicImage> {
        let mut img = self.decoded.borrow_mut();

        let mut ops = self.config.pipeline.borrow_mut();
        if !ops.is_empty() {
            for op in ops.drain(..) {
                op.apply(&mut img);
            }
        }
        drop(img);

        self.decoded.borrow()
    }
}
