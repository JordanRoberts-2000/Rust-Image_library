mod error;
mod types;
mod core {
    pub mod encoding;
    pub mod image;
    pub mod images;
    pub mod metadata;
}
pub(crate) mod constants;
mod traits;
pub(crate) mod utils;

#[cfg(feature = "tokio")]
pub mod tokio;

#[cfg(test)]
pub(crate) mod test_utils;

pub use {
    core::{
        encoding,
        image::Image,
        images::{archive_formats, Archive, FormatFilter, FromFolderConfig, Images, ImagesBuilder},
        metadata::ImageMetadata,
    },
    error::{ErrorKind, ImageError, ValidationError},
    types::{pixels, Blur, CropEdge, ImageFormat, ImageSrc, Rgb},
};
pub(crate) use {
    core::{image, images, metadata},
    error::*,
    traits::PixelFormat,
};
