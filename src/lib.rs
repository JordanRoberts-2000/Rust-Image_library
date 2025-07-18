mod enums;
mod error;
mod types {
    pub mod byte_size;
    pub mod image;
    pub mod images;
    pub mod rgb;
}
pub(crate) mod constants;
pub mod encoders;

pub mod blocking {
    pub use super::types::{image::blocking::Image, images::blocking::Images};
}

pub use {
    enums::{ColorType, CompressionType, CropEdge, ImageFormat},
    error::ImageError,
    types::{
        byte_size::ByteSize,
        image::{r#async::Image, AvifConfig, JpegConfig, WebpConfig},
        images::enums::{ArchiveFormat, CollisionStrategy},
        rgb::Rgb,
    },
};

pub(crate) use {
    error::*,
    types::{image, images},
};
