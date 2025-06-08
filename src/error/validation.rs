use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum ValidationError {
    #[error("path does not exist: {0}")]
    PathNotFound(PathBuf),

    #[error("missing file extension: `{0}`")]
    MissingExtension(PathBuf),

    #[error("path doesn't lead to a file: {0}")]
    NotAFile(PathBuf),

    #[error("path leads to a file but isn't an image file: {0}")]
    NotAnImageFile(PathBuf),
}
