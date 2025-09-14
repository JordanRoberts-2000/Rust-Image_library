use {
    crate::{image::enums::ImageData, Image},
    image::DynamicImage,
};

impl Image {
    pub(super) fn apply_transformations(&self) {
        let mut data = self.data.borrow_mut();

        let img: &mut DynamicImage = match &mut *data {
            ImageData::RawPixels(di) => di,
            _ => unreachable!("ensure_decoded guarantees RawPixels"),
        };

        let mut ops = self.config.pipeline.borrow_mut();
        for op in ops.drain(..) {
            op.apply(img);
        }
    }
}
