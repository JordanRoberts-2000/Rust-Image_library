use {
    crate::{encoding::ColorType, ImageFormat},
    std::{error::Error, io},
};

#[derive(thiserror::Error, Debug)]
#[error("failed to encode to format '{format}': {kind}")]
pub struct EncodingError {
    format: ImageFormat,
    kind: EncodingErrorKind,
}

#[derive(thiserror::Error, Debug)]
pub enum EncodingErrorKind {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    Validation(#[from] EncodingValidationError),

    #[error(transparent)]
    Encode(Box<dyn Error + Send + Sync + 'static>),
}

impl EncodingError {
    pub fn new(format: ImageFormat, kind: EncodingErrorKind) -> Self {
        Self { format, kind }
    }
}

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum EncodingValidationError {
    #[error("buffer is empty: cannot encode an image with no data")]
    EmptyBuffer,

    #[error("GIF requires at least one frame, but none were provided")]
    EmptyFrames,

    #[error("width and height must be > 0 (got {width}x{height})")]
    InvalidDimensions { width: u32, height: u32 },

    #[error("buffer size mismatch: expected {expected_len} bytes for a {width}x{height} image with {color_type}, but got {bytes_len} bytes")]
    InvalidBufferSize {
        expected_len: usize,
        bytes_len: usize,
        width: u32,
        height: u32,
        color_type: ColorType,
    },

    #[error(
        "arithmetic overflow computing expected buffer size (w*h*c = {width}*{height}*{channels})"
    )]
    ArithmeticOverflow { width: u32, height: u32, channels: u8 },

    #[error("expected buffer size {expected} bytes exceeds configured limit {limit} bytes (w*h*c = {width}*{height}*{channels})")]
    BytesExceedLimit { expected: usize, limit: usize, width: u32, height: u32, channels: u8 },

    #[error("pixel count {pixels} exceeds configured limit {limit} ({width}x{height})")]
    PixelsExceedLimit { pixels: u64, limit: u64, width: u32, height: u32 },
}
