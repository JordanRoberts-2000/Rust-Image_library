use {
    crate::{ImageError, ImageFormat, Result},
    image::ImageReader,
    std::{
        io::{BufRead, Cursor, Seek},
        path::Path,
    },
};

pub struct ImageMetadata {
    pub format: ImageFormat,
    pub width: u32,
    pub height: u32,
}

impl ImageMetadata {
    pub fn new(width: u32, height: u32, format: ImageFormat) -> Self {
        Self {
            format,
            width,
            height,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let reader = ImageReader::new(Cursor::new(bytes))
            .with_guessed_format()
            .map_err(|_| ImageError::FormatDetectionFailed)?;

        Self::from_reader(reader)
    }

    pub fn from_path(path: &Path) -> Result<Self> {
        let reader = ImageReader::open(path).map_err(|e| ImageError::Open {
            source: e,
            path: path.to_path_buf(),
        })?;

        Self::from_reader(reader)
    }

    fn from_reader<R>(reader: ImageReader<R>) -> Result<Self>
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

        Ok(Self {
            format,
            width,
            height,
        })
    }
}
