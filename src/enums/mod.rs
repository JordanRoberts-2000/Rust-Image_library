mod color_types {
    pub mod avif;
    pub mod jpeg;
    pub mod png;
    pub mod webp;
}
mod compression;
mod crop_edge;
mod format;

pub use {
    color_types::png::PngColorType, compression::CompressionType, crop_edge::CropEdge,
    format::ImageFormat,
};
