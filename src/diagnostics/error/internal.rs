use crate::RawColorType;

#[derive(thiserror::Error, Debug)]
pub enum InternalError {
    #[error("Unsupported or unknown ColorType encountered: {0:?}")]
    UnsupportedColorType(RawColorType),
}
