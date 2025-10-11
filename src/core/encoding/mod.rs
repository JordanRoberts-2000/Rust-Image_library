mod encoders {
    pub mod avif;
    pub mod jpeg;
    pub mod png;
    pub mod tiff;
    pub mod webp;
}
mod error;
pub(crate) mod macros;
pub(crate) mod utils;
mod types {
    pub mod color_type;
    pub mod compression;
    pub mod quality;
}
mod traits {
    pub mod color_type_ops;
    pub mod encoder_ops;
}

pub(crate) use traits::{
    color_type_ops::{AlphaChannelOps, ColorTypeOps, GrayscaleOps},
    encoder_ops::EncoderOps,
};
pub use {
    encoders::{avif::*, jpeg::*, png::*, tiff::*, webp::*},
    error::{EncodingError, EncodingErrorKind, EncodingValidationError},
    types::{color_type::ColorType, compression::CompressionType, quality::Quality},
};
