use {
    crate::{BitDepth, ColorModel, ImageFormat},
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

    #[error("output path `{0}` has no parent directory")]
    MissingParent(PathBuf),

    #[error("Input byte array cannot be empty")]
    EmptyByteArray,

    #[error("Index {0} out of bounds")]
    IndexOutOfBounds(usize),

    #[error("Invalid buffer: pixels could not be read for color model {0:?}")]
    InvalidBuffer(ColorModel),

    #[error("unsupported file extension: {0}")]
    UnsupportedExtension(String),

    #[error(
        "unsupported color model/bit-depth combination: model={model:?}, bit_depth={bit_depth:?}"
    )]
    UnsupportedModelBitDepth { model: ColorModel, bit_depth: BitDepth },
}
