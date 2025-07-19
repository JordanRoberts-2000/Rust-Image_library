use {
    crate::Result,
    mockall::automock,
    std::{fs::File, path::Path},
    tempfile::NamedTempFile,
};

#[automock]
pub trait FsRepoOps {
    fn check_existing_file(&self, path: &Path) -> Result<()>;
    fn check_existing_dir(&self, path: &Path) -> Result<()>;
    fn get_file_size(&self, path: &Path) -> Result<u64>;
    fn trash_file(&self, path: &Path) -> Result<()>;
    fn ensure_dir(&self, path: &Path) -> Result<()>;
    fn create_temp_file(&self, parent: &Path) -> Result<NamedTempFile>;
    fn persist_temp_file(&self, temp: NamedTempFile, path: &Path) -> Result<File>;
}
