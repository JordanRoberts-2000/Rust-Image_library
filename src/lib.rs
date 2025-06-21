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
    error::{InternalError, IoError, Result, ValidationError},
    image::{
        core::ImageConfig,
        enums::{ImageData, ImageSrc, TransformOp},
    },
};
