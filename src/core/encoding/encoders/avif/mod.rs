mod core;
mod encode;
mod types {
    pub mod color_type;
    pub mod config;
    pub mod speed;
}

pub use {
    core::AvifEncoder,
    types::{config::AvifConfig, speed::AvifSpeed},
};
