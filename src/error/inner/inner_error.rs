use {
    crate::{EncodingError, ImageFormat, ValidationError},
    std::{io, path::PathBuf},
    tokio::task::JoinError,
    url::Url,
    walkdir::Error as WalkDirError,
};

#[derive(thiserror::Error, Debug)]
pub enum InnerError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),

    #[error("Encoding error: {0}")]
    Encoding(#[from] EncodingError),

    #[error("Color type '{0:?}' not supported")]
    UnsupportedColorType(image::ColorType),

    #[error("Failed to join blocking task: {0}")]
    TaskJoinError(JoinError),

    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("Failed to decode from memory buffer: {0}")]
    DecodeFromMemory(image::ImageError),

    #[error("Failed to decode base64: {0}")]
    Base64DecodeFailed(base64::DecodeError, String),

    #[error("Failed to decode image '{id}' to format '{format:?}': {source}")]
    Decoding { id: String, source: image::ImageError, format: ImageFormat },

    #[error("Failed to open file '{path:?}': {source}")]
    Open { source: std::io::Error, path: PathBuf },

    #[error("Failed to decode file '{path:?}': {source}")]
    DecodeFile { source: image::ImageError, path: PathBuf },

    #[error("Failed to decode from reader: {0}")]
    DecodeReader(image::ImageError),

    #[error("Failed to download from '{url}': {source}")]
    DownloadFailed { url: Url, source: reqwest::Error },

    #[error("Failed to read response bytes from '{url}': {source}")]
    ResponseReadFailed { url: Url, source: reqwest::Error },

    #[error("{}", format_unsupported_error(.0))]
    UnsupportedFormat(image::ImageFormat),

    #[error("{}", ext_unsupported_error(.0))]
    InvalidExtension(String),

    #[error("Failed to detect image format from byte stream: {0}")]
    FormatDetectionFailed(std::io::Error),

    #[error("Unknown or unsupported image format")]
    UnknownFormat,

    #[error("Failed to read image dimensions: {0}")]
    DimensionsFailed(image::ImageError),

    #[error("Missing file extension on path '{0:?}'")]
    ExtensionMissing(PathBuf),

    #[error("Failed to save to '{path:?}': {source}")]
    Save { source: image::ImageError, path: PathBuf },

    #[error("Failed to retrieve color palette: {0}")]
    GetColors(color_thief::Error),

    #[error("Color palette is empty")]
    EmptyPalette,

    #[error("Failed to encode blurhash: {0}")]
    BlurHash(blurhash::Error),

    #[error("Path missing file name: '{0:?}'")]
    MissingFileName(PathBuf),

    #[error("Request failed with status {status_code}: {message}")]
    FailedRequest { url: Url, status_code: u16, message: String },

    #[error("Directory traversal error: {0}")]
    WalkDir(WalkDirError),

    #[error("source is not a local file")]
    SourceIsNotFile,

    #[error("File name collision detected: '{0}'")]
    FileNameCollision(String),
}

pub fn format_unsupported_error(format: &image::ImageFormat) -> String {
    let supported = ImageFormat::supported().join(",");
    format!("unsupported image format: '{format:?}'; supported formats are: {supported}")
}

pub fn ext_unsupported_error(ext: &String) -> String {
    let supported = ImageFormat::supported().join(",");
    format!("unsupported ext: '{ext}'; supported extentions are: {supported}")
}
