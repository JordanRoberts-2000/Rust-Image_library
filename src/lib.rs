mod enums;
mod error;
mod image;
pub(crate) mod utils;

pub use {
    enums::{CompressionType, CropEdge, ImageFormat},
    error::ImageError,
    image::core::Image,
};

pub(crate) use {
    enums::ImageSrc,
    error::{IoError, Result, ValidationError},
    image::config::ImageConfig,
};
