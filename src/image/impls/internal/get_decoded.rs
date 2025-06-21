use image::{DynamicImage, ImageReader};

use crate::{Image, ImageData, ImageError, InternalError};

impl Image {
    pub fn get_decoded(&mut self) -> Result<&mut DynamicImage, ImageError> {
        if let ImageData::Decoded(ref mut decoded) = self.data {
            return Ok(decoded);
        }

        let decoded = match &self.data {
            ImageData::File(path) => {
                let reader = ImageReader::open(path).map_err(|e| ImageError::Open {
                    source: e,
                    path: path.clone(),
                })?;
                reader.decode().map_err(|e| ImageError::DecodeFile {
                    source: e,
                    path: path.clone(),
                })?
            }
            ImageData::Bytes(bytes) => {
                image::load_from_memory_with_format(bytes, self.format.into()).map_err(|e| {
                    ImageError::Decoding {
                        id: self.describe_source(),
                        format: self.format,
                        source: e,
                    }
                })?
            }
            ImageData::Decoded(_) => {
                return Err(InternalError::DecodingInvariantViolatedBeforeDecodeMatch.into());
            }
        };

        self.data = ImageData::Decoded(decoded);

        if let ImageData::Decoded(ref mut decoded) = self.data {
            Ok(decoded)
        } else {
            Err(InternalError::DecodingInvariantViolatedAfterDecodeAssignment.into())
        }
    }
}
