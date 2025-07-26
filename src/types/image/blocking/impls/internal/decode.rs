use {
    crate::{blocking::Image, image::blocking::ImageData, utils::decode, Result},
    image::DynamicImage,
    std::cell::{Ref, RefMut},
};

impl Image {
    pub(crate) fn ensure_decoded(&self) -> Result<()> {
        let mut data = self.data.borrow_mut();

        if matches!(&*data, ImageData::DynamicImage(_)) {
            return Ok(());
        }

        let decoded = match &*data {
            ImageData::File(path) => decode::from_path(path)?,
            ImageData::EncodedBytes(bytes) => decode::from_bytes(bytes)?,
            ImageData::DynamicImage(_) => unreachable!(),
        };

        *data = ImageData::DynamicImage(decoded);
        Ok(())
    }

    pub(crate) fn get_decoded_mut(&self) -> RefMut<'_, DynamicImage> {
        let data_ref: RefMut<ImageData> = self.data.borrow_mut();
        RefMut::map(data_ref, |d| match d {
            ImageData::DynamicImage(img) => img,
            _ => unreachable!("ensure_decoded guaranteed a DynamicImage"),
        })
    }

    pub(crate) fn get_decoded(&self) -> Ref<'_, DynamicImage> {
        let data_ref: Ref<ImageData> = self.data.borrow();
        Ref::map(data_ref, |d| match d {
            ImageData::DynamicImage(img) => img,
            _ => unreachable!("ensure_decoded guaranteed a DynamicImage"),
        })
    }
}
