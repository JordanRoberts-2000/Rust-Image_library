mod enums;
mod error;
pub(crate) mod image;
pub(crate) mod utils;

pub use {
    enums::{CompressionType, CropEdge, ImageFormat},
    error::ImageError,
    image::{
        config::{AvifConfig, JpegConfig, WebpConfig},
        core::Image,
    },
};

pub(crate) use {
    error::{InternalError, IoError, Result, ValidationError},
    image::{
        config::ImageConfig,
        constants,
        enums::{ImageData, ImageSrc, TransformOp},
    },
};
