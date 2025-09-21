use {
    crate::{ImageFormat, InnerError, ValidationError},
    image::ImageReader,
    std::{
        io::{BufRead, Cursor, Seek},
        num::NonZeroU32,
        path::Path,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct ImageMetadata {
    pub format: ImageFormat,
    pub height: NonZeroU32,
    pub width: NonZeroU32,
}

impl ImageMetadata {
    pub fn new(width: u32, height: u32, format: ImageFormat) -> Result<Self, InnerError> {
        Ok(Self {
            format,
            width: NonZeroU32::new(width).ok_or(ValidationError::InvalidWidth)?,
            height: NonZeroU32::new(height).ok_or(ValidationError::InvalidHeight)?,
        })
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, InnerError> {
        let reader = ImageReader::new(Cursor::new(bytes))
            .with_guessed_format()
            .map_err(InnerError::FormatDetectionFailed)?;

        Self::from_reader(reader)
    }

    pub fn from_path(path: &Path) -> Result<Self, InnerError> {
        let reader = ImageReader::open(path)
            .map_err(|e| InnerError::Open { source: e, path: path.to_path_buf() })?;

        Self::from_reader(reader)
    }

    fn from_reader<R>(reader: ImageReader<R>) -> Result<Self, InnerError>
    where
        R: BufRead + Seek,
    {
        let format =
            reader.format().ok_or(InnerError::UnknownFormat).and_then(ImageFormat::try_from)?;

        let (width, height) = reader.into_dimensions().map_err(InnerError::DimensionsFailed)?;

        Ok(Self {
            format,
            width: NonZeroU32::new(width).ok_or(ValidationError::InvalidWidth)?,
            height: NonZeroU32::new(height).ok_or(ValidationError::InvalidHeight)?,
        })
    }
}
