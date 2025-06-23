mod enums;
mod error;
pub(crate) mod image;
pub(crate) mod utils;

pub use {
    enums::{CompressionType, CropEdge, ImageFormat},
    error::ImageError,
    image::core::Image,
};

pub(crate) use {
    error::{InternalError, IoError, Result, ValidationError},
    image::{
        constants::DEFAULT_IMAGE_FILE_NAME,
        core::ImageConfig,
        enums::{ImageData, ImageSrc, TransformOp},
    },
};
