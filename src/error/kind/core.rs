#[cfg(feature = "tokio")]
use tokio::task::JoinError;
use {
    crate::{encoding::EncodingError, ValidationError},
    std::{io, path::PathBuf},
    url::Url,
};

#[derive(thiserror::Error, Debug)]
pub enum ErrorKind {
    #[cfg(feature = "tokio")]
    #[error("Failed to join blocking task: {0}")]
    TaskJoinError(JoinError),

    #[error("internal error: {0}")]
    Internal(String),

    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),

    #[error("Encoding error: {0}")]
    Encoding(#[from] EncodingError),

    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("Failed to decode from memory buffer: {0}")]
    DecodeFromMemory(image::ImageError),

    #[error("Failed to decode base64: {0}")]
    Base64DecodeFailed(base64::DecodeError, String),

    #[error("Failed to open file '{path:?}': {source}")]
    Open { source: std::io::Error, path: PathBuf },

    #[error("Failed to decode file '{0}'")]
    Decode(image::ImageError),

    #[error("Failed to download from '{url}': {source}")]
    DownloadFailed { url: Url, source: reqwest::Error },

    #[error("Failed to read response bytes from '{url}': {source}")]
    ResponseReadFailed { url: Url, source: reqwest::Error },

    #[error("Failed to detect format: {0}")]
    FormatDetectionFailed(std::io::Error),

    #[error("Unknown or unsupported image format")]
    UnknownFormat,

    #[error("Failed to read image dimensions: {0}")]
    PeakDimensionsFailed(image::ImageError),

    #[error("Failed to save to '{path:?}': {source}")]
    Save { source: image::ImageError, path: PathBuf },

    #[error("Failed to retrieve color palette: {0}")]
    GetColors(color_thief::Error),

    #[error("Color palette is empty")]
    EmptyPalette,

    #[error("Failed to encode blurhash: {0}")]
    BlurHash(blurhash::Error),

    #[error("Request failed with status {status_code}: {message}")]
    FailedRequest { url: Url, status_code: u16, message: String },

    #[error("source is not a local file")]
    SourceIsNotFile,

    #[error("could not rasterize svg")]
    SvgRaster,

    #[error("gif had no frames")]
    EmptyGif,

    #[error("Image is static, it has no frames")]
    NotAnimated,

    #[error("frame index {index} is out of bounds (len = {len})")]
    FrameOutOfBounds { index: usize, len: usize },

    #[error("File name collision detected: '{0}'")]
    FileNameCollision(String),
}
