use {
    crate::{
        image::{
            blocking::{
                dependencies::ImageDeps,
                traits::{ImageDepsOps, MetadataOps},
            },
            enums::ImageSrc,
            r#async::{Image, ImageData, ImageState},
            ImageConfig,
        },
        ImageError, IoError, Result,
    },
    std::{
        io::{BufRead, Seek},
        sync::Arc,
    },
    tokio::{sync::RwLock, task::spawn_blocking},
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

            let state = ImageState {
                config: ImageConfig::default(),
                data: ImageData::EncodedBytes(Arc::new(bytes)),
                height,
                width,
                format,
            };

            Ok(Self {
                src: ImageSrc::Reader,
                state: Arc::new(RwLock::new(state)),
            })
        })
        .await
        .map_err(ImageError::TaskJoinError)?
    }
}
