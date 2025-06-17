use image::{GenericImageView, ImageReader};
use std::io::{BufRead, Seek};

use crate::{Image, ImageConfig, ImageError, ImageFormat, ImageSrc};

impl Image {
    pub fn from_reader(reader: impl BufRead + Seek) -> Result<Self, ImageError> {
        let reader = ImageReader::new(reader);

        let reader_fmt = reader.format().ok_or(ImageError::GuessFormat)?;
        let format = ImageFormat::try_from(reader_fmt)?;

        let raw = reader.decode().map_err(ImageError::DecodeReader)?;

        let (width, height) = raw.dimensions();

        Ok(Self {
            src: ImageSrc::Reader,
            raw,
            config: ImageConfig::default(),
            format,
            width,
            height,
            aspect_ratio: width as f32 / height as f32,
        })
    }
}
