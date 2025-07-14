use {
    crate::{
        image::{enums::ImageSrc, r#async::ImageData, ImageConfig},
        ImageFormat,
    },
    std::{num::NonZeroU32, sync::Arc},
    tokio::sync::RwLock,
};

#[derive(Debug, Clone)]
pub struct Image {
    pub(super) src: ImageSrc,
    pub(super) state: Arc<RwLock<ImageState>>,
}

#[derive(Debug, Clone)]
pub struct ImageState {
    pub config: ImageConfig,
    pub data: ImageData,
    pub height: NonZeroU32,
    pub width: NonZeroU32,
    pub format: ImageFormat,
}
