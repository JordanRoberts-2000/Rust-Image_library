#[derive(thiserror::Error, Debug)]
pub enum EncodingError {
    #[error("Failed to encode to format 'png'")]
    PngEncoding(image::ImageError),

    #[error("Failed to encode to format 'webp'")]
    WebPLossyEncoding { err: webp::WebPEncodingError },

    #[error("Failed to encode to format 'webp'")]
    WebPLosslessEncoding(image::ImageError),

    #[error("Failed to encode to format 'jpeg'")]
    JpegEncoding(image::ImageError),

    #[error("failed to encode to format 'avif'")]
    AvifEncoding { err: image::ImageError },

    #[cfg(feature = "progressive-jpeg")]
    #[error("Failed to start progressive JPEG compression for: {0}")]
    JpegCompressionStart(std::io::Error),

    #[cfg(feature = "progressive-jpeg")]
    #[error("Failed to finalize progressive JPEG compression: {0}")]
    JpegCompressionFinish(std::io::Error),

    #[cfg(feature = "progressive-jpeg")]
    #[error("Failed to write progressive JPEG to output: {0}")]
    JpegWriteOutput(std::io::Error),

    #[cfg(feature = "progressive-jpeg")]
    #[error("Failed to write JPEG scanlines{0}")]
    JpegWriteScanlines(std::io::Error),
}
