use {
    crate::{ImageFormat, Result, ValidationError},
    std::num::NonZeroU32,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ImageMetadata {
    pub format: ImageFormat,
    pub(crate) height: NonZeroU32,
    pub(crate) width: NonZeroU32,
}

impl ImageMetadata {
    pub fn new(width: u32, height: u32, format: ImageFormat) -> Result<Self> {
        Ok(Self {
            format,
            width: NonZeroU32::new(width).ok_or(ValidationError::InvalidWidth)?,
            height: NonZeroU32::new(height).ok_or(ValidationError::InvalidHeight)?,
        })
    }
}
