use {
    crate::{image::enums::ImageData, Image, Result},
    image::DynamicImage,
    std::cell::Ref,
};

impl Image {
    pub(crate) fn processed_image(&self) -> Result<Ref<'_, DynamicImage>> {
        self.ensure_decoded()?;
        self.apply_transformations();
        self.conform_color_type()?;

        Ok(Ref::map(self.data.borrow(), |d| match d {
            ImageData::RawPixels(img) => img,
            _ => unreachable!("ensure_processed guarantees pixels"),
        }))
    }
}
