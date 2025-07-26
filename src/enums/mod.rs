mod color_types {
    pub mod avif;
    pub mod png;
    pub mod raw;
    pub mod webp;
}
mod compression;
mod crop_edge;
mod format;

pub use {
    color_types::{
        avif::AvifColorType,
        png::PngColorType,
        raw::{RawColorType, RawColorTypeF32, RawColorTypeU16},
        webp::WebPColorType,
    },
    compression::CompressionType,
    crop_edge::CropEdge,
    format::ImageFormat,
};
