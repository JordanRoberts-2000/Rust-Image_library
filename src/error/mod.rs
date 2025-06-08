mod core;
mod io;
mod validation;

pub use {core::ImgError, io::IoError, validation::ValidationError};

pub type Result<T> = std::result::Result<T, ImgError>;
