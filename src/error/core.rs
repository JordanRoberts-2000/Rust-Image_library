use std::path::PathBuf;

use super::{io::IoError, validation::ValidationError};

use crate::ImageFormat;

#[derive(thiserror::Error, Debug)]
pub enum ImageError {
    #[error(transparent)]
    Io(#[from] IoError),

    #[error(transparent)]
    Validation(#[from] ValidationError),

    #[error("failed to encode img '{id}' to format '{format:?}'")]
    Encoding {
        source: image::ImageError,
        id: String,
        format: ImageFormat,
    },

    #[error("failed to encode img '{id}' to format 'webp'")]
    WebPEncoding {
        err: webp::WebPEncodingError,
        id: String,
    },

    #[error(
        "failed to decode image '{}' to format '{:?}', err: {}",
        id,
        format,
        source
    )]
    Decoding {
        id: String,
        source: image::ImageError,
        format: ImageFormat,
    },

    #[error("failed to open img '{:?}', err: {}", path, source)]
    Open {
        source: std::io::Error,
        path: PathBuf,
    },

    #[error("failed to decode image file '{:?}', err: {}", path, source)]
    DecodeFile {
        source: image::ImageError,
        path: PathBuf,
    },

    #[error("failed to decode image from reader, err: {0}")]
    DecodeReader(image::ImageError),

    #[error("Failed to download image from '{url}': {source}")]
    DownloadFailed { url: String, source: reqwest::Error },

    #[error("Failed to read bytes from response for '{url}': {source}")]
    ResponseReadFailed { url: String, source: reqwest::Error },

    #[error("{}", format_unsupported_error(.0))]
    UnsupportedFormat(image::ImageFormat),

    #[error("{}", ext_unsupported_error(.0))]
    InvalidExtension(String),

    #[error("Failed to detect image format from byte stream")]
    FormatDetectionFailed,

    #[error("Unknown or unsupported image format")]
    UnknownFormat,

    #[error("Failed to read image dimensions: {0:?}")]
    DimensionsFailed(image::ImageError),

    #[error("Extention required on path '{0}'")]
    ExtensionMissing(PathBuf),

    #[error("failed to create new image {:?}, err: {}", path, source)]
    Save {
        source: image::ImageError,
        path: PathBuf,
    },

    #[error("Could not retrieve palette: {0}")]
    GetColors(color_thief::Error),

    #[error("No colors in retrieved palette")]
    EmptyPalette,

    #[error("Blurhash failed to encode img, err: {0}")]
    BlurHash(blurhash::Error),

    #[error("Path does not have a file name: {0:?}")]
    MissingFileName(PathBuf),

    #[error("failed to parse url '{1}', {0}")]
    UrlParse(#[source] url::ParseError, String),

    #[error("'{url}' response returned status code '{status_code}'")]
    FailedRequest {
        url: String,
        status_code: u16,
        message: String,
    },

    #[error("output path `{0}` has no parent directory")]
    MissingParent(PathBuf),
}

pub fn format_unsupported_error(format: &image::ImageFormat) -> String {
    let supported = ImageFormat::supported().join(",");
    format!("unsupported image format: '{format:?}'; supported formats are: {supported}")
}

pub fn ext_unsupported_error(ext: &String) -> String {
    let supported = ImageFormat::supported().join(",");
    format!("unsupported ext: '{ext}'; supported extentions are: {supported}")
}
