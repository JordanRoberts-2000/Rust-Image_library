mod core;
mod types {
    pub mod repeat;
    pub mod speed;
}

pub use {
    core::GifEncoder,
    types::{repeat::GifRepeat, speed::GifSpeed},
};
