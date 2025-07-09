use {crate::ImageError, mockall::automock, std::path::Path};

#[automock]
pub trait FsOps {
    fn ensure_existing_file(&self, path: &Path) -> Result<(), ImageError>;
}
