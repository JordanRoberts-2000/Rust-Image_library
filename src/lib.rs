mod enums;
mod error;
mod img;
pub(crate) mod utils;

pub use {
    enums::{CompressionType, CropEdge, ImageFormat},
    error::ImgError,
    img::core::Img,
};

pub(crate) use {
    enums::ImgSrc,
    error::{IoError, Result, ValidationError},
    img::transform_pipeline::TransformPipeline,
};
