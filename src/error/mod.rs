mod error;
mod internal;
mod io;
mod validation;

pub use {error::ImageError, io::IoError, validation::ValidationError};

pub(crate) use internal::InternalError;

pub type Result<T> = std::result::Result<T, ImageError>;
