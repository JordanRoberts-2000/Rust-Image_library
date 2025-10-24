mod encoders {
    pub mod avif;
    pub mod gif;
    pub mod jpeg;
    pub mod png;
    pub mod tiff;
    pub mod webp;
}
mod error;
pub(crate) mod utils;
mod types {
    pub mod color_type;
    pub mod compression;
    pub mod quality;
}
mod traits {
    pub mod color_type_ops;
    pub mod encoder;
}

pub(crate) use traits::color_type_ops::{
    AlphaChannelOps, BitDepthOps, ColorTypeOps, EncodeColorTypeOps, GrayscaleOps,
};
pub use {
    encoders::{avif::*, gif::*, jpeg::*, png::*, tiff::*, webp::*},
    error::{EncodingError, EncodingErrorKind, EncodingValidationError},
    traits::encoder::Encoder,
    types::{
        color_type::{
            AvifColorType, ColorType, JpegColorType, PngColorType, TiffColorType, WebpColorType,
        },
        compression::CompressionType,
        quality::Quality,
    },
};
