use {
    crate::{
        image::{r#async::traits::MetadataOps, utils::parse_reader_dimensions},
        ImageError, ImageFormat, IoError, Result,
    },
    image::ImageReader,
    std::{
        io::{BufRead, Cursor, Seek},
        num::NonZeroU32,
        path::Path,
        sync::Arc,
    },
    tokio::{fs::File, io::AsyncReadExt, task::spawn_blocking},
};

pub struct Metadata;

impl MetadataOps for Metadata {
    async fn from_path(&self, path: &Path) -> Result<(ImageFormat, NonZeroU32, NonZeroU32)> {
        let mut file = File::open(path).await.map_err(|e| ImageError::Open {
            source: e,
            path: path.to_path_buf(),
        })?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .await
            .map_err(|e| IoError::ReadStream(e))?;

        self.from_bytes(Arc::new(buffer)).await
    }

    async fn from_bytes(
        &self,
        bytes: Arc<Vec<u8>>,
    ) -> Result<(ImageFormat, NonZeroU32, NonZeroU32)> {
        spawn_blocking(move || {
            let reader = ImageReader::new(Cursor::new(bytes.as_ref()))
                .with_guessed_format()
                .map_err(|_| ImageError::FormatDetectionFailed)?;

            parse_reader_dimensions(reader)
        })
        .await
        .map_err(ImageError::TaskJoinError)?
    }

    async fn from_reader<R>(&self, reader: R) -> Result<(ImageFormat, NonZeroU32, NonZeroU32)>
    where
        R: BufRead + Seek + Send + 'static,
    {
        spawn_blocking(move || {
            let image_reader = ImageReader::new(reader)
                .with_guessed_format()
                .map_err(|_| ImageError::FormatDetectionFailed)?;

            parse_reader_dimensions(image_reader)
        })
        .await
        .map_err(ImageError::TaskJoinError)?
    }
}
