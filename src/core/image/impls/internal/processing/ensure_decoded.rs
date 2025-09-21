use {
    crate::{image::ImageData, utils::decode, Image, Result},
    image::DynamicImage,
    std::cell::RefMut,
};

impl Image {
    pub(super) fn ensure_decoded(&self) -> Result<RefMut<'_, DynamicImage>> {
        let mut data = self.data.borrow_mut();

        if !matches!(*data, ImageData::RawPixels(_)) {
            let decoded = match &*data {
                ImageData::File(path) => decode::from_path(path)?,
                ImageData::EncodedBytes(bytes) => decode::from_bytes(bytes)?,
                ImageData::RawPixels(_) => unreachable!(),
            };
            *data = ImageData::RawPixels(decoded);
        }

        Ok(RefMut::map(data, |d| match d {
            ImageData::RawPixels(img) => img,
            _ => unreachable!(),
        }))
    }
}
