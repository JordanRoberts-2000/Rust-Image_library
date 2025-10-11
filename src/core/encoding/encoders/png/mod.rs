mod core;
mod encode;
mod types {
    pub mod color_type;
    pub mod compression;
    pub mod config;
}

pub use {
    core::PngEncoder,
    types::{color_type::PngColorType, compression::PngCompressionType, config::PngConfig},
};
