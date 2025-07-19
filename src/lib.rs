mod enums;
mod error;
mod types {
    pub mod byte_size;
    // pub mod image;
    // pub mod images;
    pub mod metadata;
    pub mod rgb;
}
pub mod blocking;
pub(crate) mod constants;
pub mod encoders;

pub use {
    enums::{CompressionType, CropEdge, ImageFormat},
    error::ImageError,
    types::{
        byte_size::ByteSize,
        // image::{r#async::Image, AvifConfig, JpegConfig, WebpConfig},
        // images::enums::{ArchiveFormat, CollisionStrategy},
        metadata::ImageMetadata,
        rgb::Rgb,
    },
};

pub(crate) use {
    error::*,
    // types::{image, images},
};
