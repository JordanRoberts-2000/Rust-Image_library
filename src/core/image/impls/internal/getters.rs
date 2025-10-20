use {
    crate::{image::Decoded, Image, ImageSrc, Result},
    image::DynamicImage,
    std::{borrow::Cow, cell::Ref},
};

impl Image {
    pub(crate) fn src(&self) -> ImageSrc {
        self.src.clone()
    }

    pub(crate) fn processed_decode(&self) -> Ref<Decoded> {
        let mut decoded = self.decoded.borrow_mut();

        let mut ops = self.config.pipeline.borrow_mut();
        if !ops.is_empty() {
            for op in ops.drain(..) {
                op.apply(&mut decoded);
            }
        }
        drop(decoded);

        self.decoded.borrow()
    }
}
