mod encoders {
    pub mod avif;
    pub mod jpeg;
    pub mod png;
    pub mod webp;
}
mod error;
pub(crate) mod macros;
pub(crate) mod utils;
mod types {
    pub mod bit_depth;
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
    encoders::{avif::*, jpeg::*, png::*, webp::*},
    error::{EncodingError, EncodingErrorKind, EncodingValidationError},
    types::{
        bit_depth::BitDepth, color_type::ColorType, compression::CompressionType, quality::Quality,
    },
};
