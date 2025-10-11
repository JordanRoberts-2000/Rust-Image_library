use crate::encoding::{ColorType, EncodingErrorKind, EncodingValidationError};

pub fn validate_buffer(
    bytes: &[u8], width: u32, height: u32, color_type: impl Into<ColorType>,
) -> Result<(), EncodingErrorKind> {
    let color_type = color_type.into();

    if width == 0 || height == 0 {
        return Err(EncodingValidationError::InvalidDimensions { width, height }.into());
    }

    if bytes.is_empty() {
        return Err(EncodingValidationError::EmptyBuffer.into());
    }

    let channels = color_type.channels();
    let expected_len = (width as usize)
        .checked_mul(height as usize)
        .and_then(|p| p.checked_mul(channels as usize))
        .ok_or(EncodingValidationError::ArithmeticOverflow { width, height, channels })?;

    if bytes.len() != expected_len {
        return Err(EncodingValidationError::InvalidBufferSize {
            expected_len,
            bytes_len: bytes.len(),
            width,
            height,
            color_type: color_type.into(),
        }
        .into());
    }

    Ok(())
}
