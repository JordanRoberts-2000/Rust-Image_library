mod color_types {
    pub mod avif;
    pub mod common;
    pub mod jpeg;
    pub mod png;
    pub mod webp;
}
mod bit_depth;
mod compression;
mod crop_edge;
mod format;

pub use {
    bit_depth::BitDepth,
    color_types::{
        avif::AvifColorType,
        common::{ColorModel, ColorType},
        jpeg::JpegColorType,
        png::PngColorType,
        webp::WebPColorType,
    },
    compression::{CompressionType, PngCompressionType},
    crop_edge::CropEdge,
    format::ImageFormat,
};
