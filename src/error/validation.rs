use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum ValidationError {
    #[error("path does not exist: {0}")]
    PathNotFound(PathBuf),

    #[error("path doesn't lead to a file: {0}")]
    NotAFile(PathBuf),
}
