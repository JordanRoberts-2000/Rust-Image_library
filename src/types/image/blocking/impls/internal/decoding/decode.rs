use image::{DynamicImage, ImageReader};

use crate::{blocking::Image, image::blocking::ImageData, ImageError, InternalError};

impl Image {
    pub(crate) fn decode(&self) -> Result<DynamicImage, ImageError> {
        match &self.data {
            ImageData::File(path) => {
                let reader = ImageReader::open(path).map_err(|e| ImageError::Open {
                    source: e,
                    path: path.clone(),
                })?;

                reader.decode().map_err(|e| ImageError::DecodeFile {
                    source: e,
                    path: path.clone(),
                })
            }

            ImageData::EncodedBytes(bytes) => {
                image::load_from_memory_with_format(bytes, self.format.into()).map_err(|e| {
                    ImageError::Decoding {
                        id: self.describe_source(),
                        format: self.format,
                        source: e,
                    }
                })
            }

            ImageData::Decoded(_) => {
                Err(InternalError::DecodingInvariantViolatedBeforeDecodeMatch.into())
            }
        }
    }
}
