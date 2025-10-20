use {
    crate::{encoding::ColorType, Format, ImageFormat},
    std::path::PathBuf,
};

#[derive(thiserror::Error, Debug)]
pub enum ValidationError {
    #[error("path does not exist: {0}")]
    PathNotFound(PathBuf),

    #[error("path doesn't lead to a directory: {0}")]
    NotADirectory(PathBuf),

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

    #[error("Invalid buffer: pixels could not be read with color-type '{0:?}'")]
    InvalidBuffer(ColorType),

    #[error("Color type '{0:?}' not supported")]
    UnsupportedColorType(image::ColorType),

    #[error("{}", ext_unsupported_error(.0))]
    UnsupportedExtension(String),

    #[error("{}", format_unsupported_error(.0))]
    UnsupportedFormat(image::ImageFormat),

    #[error("{}", image_format_unsupported_error(.0))]
    UnsupportedImageFormat(image::ImageFormat),

    #[error("Metadata is not supported for format {0:?}")]
    UnsupportedMetadataFormat(Format),

    #[error("Missing file extension on path '{0:?}'")]
    MissingExtension(PathBuf),

    #[error("Path missing file name: '{0:?}'")]
    MissingFileName(PathBuf),
}

fn ext_unsupported_error(ext: &String) -> String {
    let supported = ImageFormat::supported_exts().join(",");
    format!("unsupported ext: '{ext}'; supported extentions are: {supported}")
}

fn format_unsupported_error(format: &image::ImageFormat) -> String {
    let supported = Format::supported_exts().join(",");
    format!("unsupported format: '{format:?}'; supported formats are: {supported}")
}

fn image_format_unsupported_error(format: &image::ImageFormat) -> String {
    let supported = ImageFormat::supported_exts().join(",");
    format!("unsupported image format: '{format:?}'; supported formats are: {supported}")
}
