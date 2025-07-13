mod enums;
mod error;
mod types {
    pub mod byte_size;
    pub mod image;
    pub mod rgb;
}
pub(crate) mod constants;

pub mod blocking {
    pub use super::types::image::blocking::Image;
}

pub use {
    enums::{ColorType, CompressionType, CropEdge, ImageFormat},
    error::ImageError,
    types::{
        byte_size::ByteSize,
        image::{AvifConfig, JpegConfig, WebpConfig},
        rgb::Rgb,
    },
};

pub(crate) use {error::*, types::image};
