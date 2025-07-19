mod encoding;
mod error;
mod internal;
mod io;
mod validation;

pub use {
    encoding::EncodingError, error::ImageError, internal::InternalError, io::IoError,
    validation::ValidationError,
};

pub type Result<T> = std::result::Result<T, ImageError>;
