mod error;
mod types;
mod core {
    pub mod encoders;
    pub mod image;
    // pub mod images;
}
pub(crate) mod constants;
pub mod utils;

#[cfg(test)]
pub(crate) mod test_utils;

pub(crate) use {core::image, error::*};
pub use {
    core::{
        encoders::{AvifEncoder, JpegEncoder, PngEncoder, WebPEncoder},
        image::{AvifConfig, Image, JpegConfig, PngConfig, WebpConfig},
        // images::enums::{ArchiveFormat, CollisionStrategy},
    },
    error::{ImageError, ValidationError},
    types::{
        AvifColorType, BitDepth, ColorModel, ColorType, CompressionType, CropEdge, ImageFormat,
        JpegColorType, PngColorType, PngCompressionType, Rgb, WebPColorType,
    },
};
