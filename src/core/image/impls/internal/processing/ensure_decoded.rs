use crate::{image::enums::ImageData, utils::decode, Image, Result};

impl Image {
    pub(super) fn ensure_decoded(&self) -> Result<()> {
        let mut data = self.data.borrow_mut();

        if matches!(&*data, ImageData::RawPixels(_)) {
            return Ok(());
        }

        let decoded = match &*data {
            ImageData::File(path) => decode::from_path(path)?,
            ImageData::EncodedBytes(bytes) => decode::from_bytes(bytes)?,
            ImageData::RawPixels(_) => unreachable!(),
        };

        *data = ImageData::RawPixels(decoded);
        Ok(())
    }
}
