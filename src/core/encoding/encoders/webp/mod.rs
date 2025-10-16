mod core;
mod encode;
mod types {
    pub mod color_type;
    pub mod config;
}

pub use {core::WebpEncoder, types::config::WebpConfig};
