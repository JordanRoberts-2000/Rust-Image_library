use {
    crate::{image::blocking::traits::MetadataOps, ImageError, ImageFormat},
    image::ImageReader,
    std::io::{BufRead, Cursor, Seek},
    std::path::Path,
};

pub struct Metadata;

impl MetadataOps for Metadata {
    fn from_path(&self, path: &Path) -> Result<(ImageFormat, u32, u32), ImageError> {
        let reader = ImageReader::open(path).map_err(|e| ImageError::Open {
            source: e,
            path: path.to_path_buf(),
        })?;

        let format = reader
            .format()
            .ok_or(ImageError::UnknownFormat)
            .and_then(ImageFormat::try_from)?;

        let (width, height) = reader
            .into_dimensions()
            .map_err(ImageError::DimensionsFailed)?;

        Ok((format, width, height))
    }

    fn from_bytes(&self, bytes: &Vec<u8>) -> Result<(ImageFormat, u32, u32), ImageError> {
        let reader = ImageReader::new(Cursor::new(bytes))
            .with_guessed_format()
            .map_err(|_| ImageError::FormatDetectionFailed)?;

        let format =
            ImageFormat::try_from(reader.format().ok_or_else(|| ImageError::UnknownFormat)?)?;

        let (width, height) = reader
            .into_dimensions()
            .map_err(ImageError::DimensionsFailed)?;

        Ok((format, width, height))
    }

    fn from_reader<R>(&self, reader: &mut R) -> Result<(ImageFormat, u32, u32), ImageError>
    where
        R: BufRead + Seek + 'static,
    {
        let image_reader = ImageReader::new(reader)
            .with_guessed_format()
            .map_err(|_| ImageError::FormatDetectionFailed)?;

        let format = ImageFormat::try_from(
            image_reader
                .format()
                .ok_or_else(|| ImageError::UnknownFormat)?,
        )?;

        let (width, height) = image_reader
            .into_dimensions()
            .map_err(ImageError::DimensionsFailed)?;

        Ok((format, width, height))
    }
}
