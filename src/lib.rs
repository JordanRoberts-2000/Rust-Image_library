mod enums;
mod diagnostics {
    pub mod error;
    pub mod warnings;
}
pub(crate) mod traits;
mod types {
    pub mod byte_size;
    pub mod encoders;
    pub mod image;
    pub mod images;
    pub mod metadata;
    pub mod rgb;
}
pub mod blocking;
pub(crate) mod constants;
pub(crate) mod utils {
    pub mod decode;
}

pub use {
    diagnostics::error::ImageError,
    enums::{
        CompressionType, CropEdge, ImageFormat, RawColorType, RawColorTypeF32, RawColorTypeU16,
    },
    types::{
        byte_size::ByteSize,
        encoders,
        image::{AvifConfig, JpegConfig, WebpConfig},
        images::enums::{ArchiveFormat, CollisionStrategy},
        metadata::ImageMetadata,
        rgb::Rgb,
    },
};
pub(crate) use {
    diagnostics::error::*,
    types::{image, images},
};
