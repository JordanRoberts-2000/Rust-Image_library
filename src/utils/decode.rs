use {
    crate::{ImageFormat, InnerError, ValidationError},
    image::{load_from_memory, load_from_memory_with_format, DynamicImage, ImageReader},
    std::path::Path,
};

pub fn from_path(path: impl AsRef<Path>) -> Result<DynamicImage, InnerError> {
    let path = path.as_ref();

    let reader = ImageReader::open(path)
        .map_err(|e| InnerError::Open { source: e, path: path.to_path_buf() })?
        .with_guessed_format()
        .map_err(InnerError::FormatDetectionFailed)?;

    reader.decode().map_err(|e| InnerError::DecodeFile { source: e, path: path.to_path_buf() })
}

pub fn from_bytes(bytes: &[u8]) -> Result<DynamicImage, InnerError> {
    load_from_memory(bytes).map_err(InnerError::DecodeFromMemory)
}

pub fn from_bytes_with_format(
    bytes: &[u8], format: ImageFormat,
) -> Result<DynamicImage, InnerError> {
    if let Ok(detected) = image::guess_format(bytes) {
        if detected != format.into() {
            return Err(ValidationError::FormatMismatch { expected: format, detected }.into());
        }
    }

    load_from_memory_with_format(bytes, format.into()).map_err(InnerError::DecodeFromMemory)
}
