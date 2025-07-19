use image::ImageError;

#[derive(thiserror::Error, Debug)]
pub enum EncodingError {
    #[error("Failed to encode PNG")]
    PngEncoding(ImageError),

    #[error("Failed to encode JPEG")]
    JpegEncoding(image::ImageError),

    #[error("ColorType {color} is not supported for format: {format}")]
    UnsupportedColorType { format: &'static str, color: String },

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
    // #[error("failed to encode img '{id}' to format 'webp'")]
    // WebPEncoding {
    //     err: webp::WebPEncodingError,
    //     id: String,
    // },

    // #[error("failed to encode img '{id}' to format 'avif'")]
    // AvifEncoding { err: ravif::Error, id: String },
}
