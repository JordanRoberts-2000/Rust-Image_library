mod inner {
    pub mod encoding;
    pub mod inner_error;
    pub mod validation;
}
mod error;
mod kind;
mod traits;

pub use {
    error::ImageError,
    inner::{encoding::EncodingError, inner_error::InnerError, validation::ValidationError},
    kind::ErrorKind,
    traits::ResultCtx,
};

pub type Result<T> = std::result::Result<T, ImageError>;
