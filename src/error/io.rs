use std::{io, path::PathBuf};

#[derive(thiserror::Error, Debug)]
pub enum IoError {
    #[error("Failed to create directory at `{1}`: {0}")]
    CreateDir(#[source] io::Error, PathBuf),

    #[error("Failed to create file at `{1}`: {0}")]
    CreateFile(#[source] io::Error, PathBuf),

    #[error("Failed to write file at `{1}`: {0}")]
    WriteFile(#[source] io::Error, PathBuf),

    #[error("Failed to delete file at `{1}`: {0}")]
    DeleteFile(#[source] io::Error, PathBuf),

    #[error("Failed to rename directory from `{1}` to `{2}`: {0}")]
    Rename(#[source] io::Error, PathBuf, PathBuf),

    #[error("Failed to read file at `{1}`: {0}")]
    ReadFile(#[source] io::Error, PathBuf),

    #[error("Failed to create temporary file in directory: {1}")]
    CreateTempFile(#[source] std::io::Error, PathBuf),

    #[error("Failed to persist temporary file to: {1}")]
    PersistTempFile(#[source] std::io::Error, PathBuf),
}
