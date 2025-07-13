use {
    crate::{
        image::{blocking::traits::MetadataOps, utils::parse_reader_dimensions},
        ImageError, ImageFormat,
    },
    image::ImageReader,
    std::{
        io::{BufRead, Cursor, Seek},
        num::NonZeroU32,
        path::Path,
    },
};

pub struct Metadata;

impl MetadataOps for Metadata {
    fn from_path(&self, path: &Path) -> Result<(ImageFormat, NonZeroU32, NonZeroU32), ImageError> {
        let reader = ImageReader::open(path).map_err(|e| ImageError::Open {
            source: e,
            path: path.to_path_buf(),
        })?;

        parse_reader_dimensions(reader)
    }

    fn from_bytes(
        &self,
        bytes: &Vec<u8>,
    ) -> Result<(ImageFormat, NonZeroU32, NonZeroU32), ImageError> {
        let reader = ImageReader::new(Cursor::new(bytes))
            .with_guessed_format()
            .map_err(|_| ImageError::FormatDetectionFailed)?;

        parse_reader_dimensions(reader)
    }

    fn from_reader<R>(
        &self,
        reader: &mut R,
    ) -> Result<(ImageFormat, NonZeroU32, NonZeroU32), ImageError>
    where
        R: BufRead + Seek + 'static,
    {
        let image_reader = ImageReader::new(reader)
            .with_guessed_format()
            .map_err(|_| ImageError::FormatDetectionFailed)?;

        parse_reader_dimensions(image_reader)
    }
}
