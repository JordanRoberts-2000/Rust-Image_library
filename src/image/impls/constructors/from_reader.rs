use image::ImageReader;
use std::io::{BufRead, Seek};

use crate::{Image, ImageConfig, ImageError, ImageFormat, ImageSrc};

impl Image {
    pub fn from_reader(reader: impl BufRead + Seek) -> Result<Self, ImageError> {
        let reader = ImageReader::new(reader);

        let format =
            ImageFormat::try_from(reader.format().ok_or_else(|| ImageError::UnknownFormat)?)?;

        let (width, height) = reader
            .into_dimensions()
            .map_err(ImageError::DimensionsFailed)?;

        Ok(Self {
            src: ImageSrc::Reader,
            config: ImageConfig::default(),
            format,
            width,
            height,
            aspect_ratio: width as f32 / height as f32,
        })
    }
}
