mod encoding;
mod error;
mod validation;

pub use {encoding::EncodingError, error::ImageError, validation::ValidationError};

pub type Result<T> = std::result::Result<T, ImageError>;
