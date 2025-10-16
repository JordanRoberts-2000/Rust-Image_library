mod core;
mod encode;
mod types {
    pub mod color_type;
    pub mod compression;
    pub mod config;
}

pub use {
    core::PngEncoder,
    types::{compression::PngCompressionType, config::PngConfig},
};
