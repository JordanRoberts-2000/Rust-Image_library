use {crate::Result, std::path::Path, tempfile::NamedTempFile};

pub trait FsRepoOps {
    fn check_existing_file(&self, path: &Path) -> Result<()>;
    fn check_existing_dir(&self, path: &Path) -> Result<()>;
    fn get_file_size(&self, path: &Path) -> Result<u64>;
    fn trash_file(&self, path: &Path) -> Result<()>;
    fn ensure_dir(&self, path: &Path) -> Result<()>;
    fn atomic_write<F>(&self, path: &Path, write_fn: F) -> Result<()>
    where
        F: FnOnce(&mut NamedTempFile) -> Result<()>;
}
