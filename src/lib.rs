pub(crate) mod constants;
pub(crate) mod enums;
mod error;
mod img;
pub(crate) mod utils;

pub use {error::ImgError, img::core::Img};

pub(crate) use error::{IoError, Result, ValidationError};
