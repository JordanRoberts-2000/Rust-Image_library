mod core;
mod types {
    pub mod config;
    pub mod repeat;
    pub mod speed;
}

pub use {
    core::GifEncoder,
    types::{config::GifConfig, repeat::GifRepeat, speed::GifSpeed},
};
