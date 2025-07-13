use {
    crate::Result,
    std::{fs::File, path::Path},
    tempfile::NamedTempFile,
};

pub trait FsRepoOps {
    async fn check_existing_file(&self, path: &Path) -> Result<()>;
    async fn check_existing_dir(&self, path: &Path) -> Result<()>;
    async fn get_file_size(&self, path: &Path) -> Result<u64>;
    async fn trash_file(&self, path: &Path) -> Result<()>;
    async fn ensure_dir(&self, path: &Path) -> Result<()>;
    async fn create_temp_file(&self, parent: &Path) -> Result<NamedTempFile>;
    async fn persist_temp_file(&self, temp: NamedTempFile, path: &Path) -> Result<File>;
}
