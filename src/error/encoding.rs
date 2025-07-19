use image::ImageError;

#[derive(thiserror::Error, Debug)]
pub enum EncodingError {
    #[error("Failed to encode PNG")]
    PngEncoding(ImageError),
    // #[error("failed to encode img '{id}' to format 'webp'")]
    // WebPEncoding {
    //     err: webp::WebPEncodingError,
    //     id: String,
    // },

    // #[error("failed to encode img '{id}' to format 'avif'")]
    // AvifEncoding { err: ravif::Error, id: String },
}
