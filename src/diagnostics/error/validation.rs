use {
    crate::{ImageFormat, RawColorType, RawColorTypeF32, RawColorTypeU16},
    std::path::PathBuf,
};

#[derive(thiserror::Error, Debug)]
pub enum ValidationError {
    #[error("path does not exist: {0}")]
    PathNotFound(PathBuf),

    #[error("path doesn't lead to a directory: {0}")]
    NotADirectory(PathBuf),

    #[error("missing file extension: `{0}`")]
    MissingExtension(PathBuf),

    #[error("path doesn't lead to a file: {0}")]
    NotAFile(PathBuf),

    #[error("path leads to a file but isn't an image file: {0}")]
    NotAnImageFile(PathBuf),

    #[error("Invalid image dimensions, height cannot be 0")]
    InvalidHeight,

    #[error("invalid extension format (contains invalid UTF-8): {0:?}")]
    InvalidExtensionFormat(std::ffi::OsString),

    #[error("missing file extension for path: {0}")]
    MissingExtensionForPath(PathBuf),

    #[error("Invalid image dimensions, width cannot be 0")]
    InvalidWidth,

    #[error("format mismatch: expected {expected:?}, detected {detected:?}")]
    FormatMismatch { expected: ImageFormat, detected: image::ImageFormat },

    #[error("Input byte array cannot be empty")]
    EmptyByteArray,

    #[error("Index {0} out of bounds")]
    IndexOutOfBounds(usize),

    #[error("Invalid u8 buffer: pixels could not be read for color type {0:?}")]
    InvalidBuffer(RawColorType),

    #[error("Invalid u16 buffer: pixels could not be interpreted for color type {0:?}")]
    InvalidBufferU16(RawColorTypeU16),

    #[error("Invalid f32 buffer: pixel data did not match expected layout for color type {0:?}")]
    InvalidBufferF32(RawColorTypeF32),

    #[error("unsupported file extension: {0}")]
    UnsupportedExtension(String),
}
