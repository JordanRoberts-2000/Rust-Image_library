mod core;
mod impls {
    mod encode;
    mod encode_progressive;
}
mod types {
    pub mod color_type;
    pub mod config;
}

pub use {
    core::JpegEncoder,
    types::{color_type::JpegColorType, config::JpegConfig},
};
