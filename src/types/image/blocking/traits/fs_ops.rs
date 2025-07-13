use {crate::Result, mockall::automock, std::path::Path};

#[automock]
pub trait FsOps {
    fn ensure_existing_file(&self, path: &Path) -> Result<()>;
    fn get_file_size(&self, path: &Path) -> Result<u64>;
}
