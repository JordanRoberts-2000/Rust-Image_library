use std::{
    io::{BufRead, Seek},
    num::NonZeroU32,
};

use image::ImageReader;

use crate::{ImageError, ImageFormat, Result, ValidationError};

pub fn parse_reader_dimensions<R>(
    reader: ImageReader<R>,
) -> Result<(ImageFormat, NonZeroU32, NonZeroU32)>
where
    R: BufRead + Seek,
{
    let format = reader
        .format()
        .ok_or(ImageError::UnknownFormat)
        .and_then(ImageFormat::try_from)?;

    let (width, height) = reader
        .into_dimensions()
        .map_err(ImageError::DimensionsFailed)?;

    let width = NonZeroU32::new(width).ok_or(ValidationError::InvalidDimensions(width, height))?;
    let height =
        NonZeroU32::new(height).ok_or(ValidationError::InvalidDimensions(width.get(), height))?;

    Ok((format, width, height))
}
