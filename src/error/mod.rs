mod kind {
    pub mod core;
    pub mod encoding;
    pub mod validation;
}
mod error;
mod traits;

pub use {
    error::ImageError,
    kind::{core::ErrorKind, encoding::EncodingError, validation::ValidationError},
    traits::WithSrc,
};

pub type Result<T> = std::result::Result<T, ImageError>;
