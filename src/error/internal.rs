#[derive(thiserror::Error, Debug)]
pub enum InternalError {
    #[error("Internal error: ImageData was unexpectedly already decoded before decode()")]
    DecodingInvariantViolatedBeforeDecodeMatch,

    #[error("Internal error: ImageData was expected to be Decoded after assignment but was not")]
    DecodingInvariantViolatedAfterDecodeAssignment,

    #[error("Internal error: Failed to unwrap Arc - unexpected multiple references, {0}")]
    ArcUnwrapFailed(String),
}
