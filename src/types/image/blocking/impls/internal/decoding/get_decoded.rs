use image::DynamicImage;

use crate::{blocking::Image, image::blocking::ImageData, ImageError, InternalError};

impl Image {
    pub(crate) fn get_decoded(&mut self) -> Result<&mut DynamicImage, ImageError> {
        if let ImageData::Decoded(ref mut decoded) = self.data {
            return Ok(decoded);
        }

        let decoded = self.decode()?;
        self.data = ImageData::Decoded(decoded);

        match &mut self.data {
            ImageData::Decoded(decoded) => Ok(decoded),
            _ => Err(InternalError::DecodingInvariantViolatedAfterDecodeAssignment.into()),
        }
    }
}
