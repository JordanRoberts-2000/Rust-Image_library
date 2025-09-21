use {
    crate::{image::ImageData, Image, Result},
    image::DynamicImage,
    std::cell::Ref,
};

impl Image {
    pub(crate) fn processed_image(&self) -> Result<Ref<'_, DynamicImage>> {
        let mut decoded = self.ensure_decoded()?;
        self.apply_transformations(&mut *decoded);
        drop(decoded);

        Ok(Ref::map(self.data.borrow(), |d| match d {
            ImageData::RawPixels(img) => img,
            _ => unreachable!("ensure_decoded guarantees pixels"),
        }))
    }
}
