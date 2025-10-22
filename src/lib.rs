mod error;
mod types;
mod core {
    pub mod encoding;
    pub mod format_detection;
    pub mod image;
    pub mod images;
    pub mod metadata;
}
pub(crate) mod constants;
mod traits {
    pub mod dynamic_img_ext;
    pub mod format_ops;
    pub mod pixel_format;
}
pub(crate) mod utils;

#[cfg(feature = "tokio")]
pub mod tokio;

#[cfg(test)]
pub(crate) mod test_utils;

pub use {
    core::{
        encoding, format_detection,
        image::Image,
        images::{archive_formats, Archive, FormatFilter, FromFolderConfig, Images, ImagesBuilder},
        metadata::ImageMetadata,
    },
    error::{ErrorKind, ImageError, ValidationError},
    types::{pixels, Blur, CropEdge, EncodeFormat, Format, ImageFormat, ImageSrc, Rgb},
};
pub(crate) use {
    core::{image, images},
    error::*,
    traits::{dynamic_img_ext::DynamicImageExt, format_ops::FormatOps, pixel_format::PixelFormat},
};
