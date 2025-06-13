use std::path::PathBuf;

use super::{io::IoError, validation::ValidationError};

use crate::ImageFormat;

#[derive(thiserror::Error, Debug)]
pub enum ImgError {
    #[error(transparent)]
    Io(#[from] IoError),

    #[error(transparent)]
    Validation(#[from] ValidationError),

    #[error("failed to convert img '{id}' to format '{format:?}'")]
    Conversion {
        source: image::ImageError,
        id: String,
        format: ImageFormat,
    },

    #[error("failed to convert img '{id}' to format 'webp'")]
    WebPConversion {
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
        source: image::ImageError,
        path: PathBuf,
    },

    #[error("Failed to download image from '{url}': {source}")]
    DownloadFailed { url: String, source: reqwest::Error },

    #[error("Failed to read bytes from response for '{url}': {source}")]
    ResponseReadFailed { url: String, source: reqwest::Error },

    #[error("{}", format_unsupported_error(.0))]
    UnsupportedFormat(image::ImageFormat),

    #[error("failed to retrieve file format")]
    GuessFormat,

    #[error("No valid extentions found")]
    ExtensionInvalid,

    #[error("failed to create new image {:?}, err: {}", output, source)]
    Save {
        source: image::ImageError,
        output: PathBuf,
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
}

pub fn format_unsupported_error(format: &image::ImageFormat) -> String {
    let supported = ImageFormat::supported().join(",");
    format!("unsupported image format: '{format:?}'; supported formats are: {supported}")
}
