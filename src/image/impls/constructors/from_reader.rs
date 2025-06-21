use image::ImageReader;
use std::io::{BufRead, Cursor, Seek};

use crate::{Image, ImageConfig, ImageData, ImageError, ImageFormat, ImageSrc, IoError};

impl Image {
    pub fn from_reader(mut reader: impl BufRead + Seek) -> Result<Self, ImageError> {
        let mut bytes = Vec::new();
        reader
            .read_to_end(&mut bytes)
            .map_err(IoError::ReadStream)?;

        let reader = ImageReader::new(Cursor::new(&bytes))
            .with_guessed_format()
            .map_err(|_| ImageError::FormatDetectionFailed)?;

        let format =
            ImageFormat::try_from(reader.format().ok_or_else(|| ImageError::UnknownFormat)?)?;

        let (width, height) = reader
            .into_dimensions()
            .map_err(ImageError::DimensionsFailed)?;

        Ok(Self {
            src: ImageSrc::Reader,
            data: ImageData::Bytes(bytes),
            config: ImageConfig::default(),
            height,
            width,
            aspect_ratio: width as f32 / height as f32,
            format,
        })
    }
}
