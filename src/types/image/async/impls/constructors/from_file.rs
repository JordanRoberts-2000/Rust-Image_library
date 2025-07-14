use {
    crate::{
        image::{
            enums::ImageSrc,
            r#async::{
                dependencies::ImageDeps,
                traits::{FsRepoOps, ImageDepsOps, MetadataOps},
                ImageData, ImageState,
            },
            utils::file_info,
            ImageConfig,
        },
        Image, Result,
    },
    std::{path::Path, sync::Arc},
    tokio::sync::RwLock,
};

impl Image {
    pub async fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        Self::from_file_internal(path, &ImageDeps::default()).await
    }

    async fn from_file_internal(path: &Path, image_deps: &impl ImageDepsOps) -> Result<Self> {
        image_deps.fs().check_existing_file(path).await?;

        let (format, width, height) = image_deps.metadata().from_path(path).await?;
        let (file_name, parent_dir) = file_info(path);

        let state = ImageState {
            data: ImageData::File(path.to_path_buf()),
            config: ImageConfig {
                file_name,
                output_dir: parent_dir,
                ..Default::default()
            },
            height,
            width,
            format,
        };

        Ok(Self {
            src: ImageSrc::File(path.to_path_buf()),
            state: Arc::new(RwLock::new(state)),
        })
    }
}
