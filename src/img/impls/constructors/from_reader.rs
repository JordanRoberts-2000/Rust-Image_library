use image::{GenericImageView, ImageReader};
use std::io::{BufRead, Seek};

use crate::{ImageFormat, Img, ImgError, ImgSrc};

impl Img {
    pub fn from_reader(reader: impl BufRead + Seek) -> Result<Self, ImgError> {
        let reader = ImageReader::new(reader);

        let reader_fmt = reader.format().ok_or(ImgError::GuessFormat)?;
        let format = ImageFormat::try_from(reader_fmt)?;

        let img = reader.decode().map_err(ImgError::DecodeReader)?;

        let (width, height) = img.dimensions();

        Ok(Self {
            src: ImgSrc::Reader,
            img,
            format,
            width,
            height,
            aspect_ratio: width as f32 / height as f32,
        })
    }
}
