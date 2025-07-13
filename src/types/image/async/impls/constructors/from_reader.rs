use {
    crate::{
        image::{
            blocking::{
                dependencies::ImageDeps,
                traits::{ImageDepsOps, MetadataOps},
            },
            enums::{ImageData, ImageSrc},
            r#async::Image,
            ImageConfig,
        },
        ImageError, IoError, Result,
    },
    std::io::{BufRead, Seek},
    tokio::task::spawn_blocking,
};

impl Image {
    pub async fn from_reader<R>(mut reader: R) -> Result<Self>
    where
        R: BufRead + Seek + Send + 'static,
    {
        let image_deps = ImageDeps::default();
        spawn_blocking(move || {
            reader.rewind().map_err(IoError::ReadStream)?;
            let (format, width, height) = image_deps.metadata().from_reader(reader.by_ref())?;
            reader.rewind().map_err(IoError::ReadStream)?;

            let mut bytes = Vec::new();
            reader
                .read_to_end(&mut bytes)
                .map_err(IoError::ReadStream)?;

            Ok(Self {
                src: ImageSrc::Reader,
                data: ImageData::EncodedBytes(bytes),
                config: ImageConfig::default(),
                height,
                width,
                format,
            })
        })
        .await
        .map_err(ImageError::TaskJoinError)?
    }
}
