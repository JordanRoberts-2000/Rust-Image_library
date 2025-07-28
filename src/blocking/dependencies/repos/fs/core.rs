use {
    crate::{blocking::traits::FsRepoOps, Result},
    std::path::Path,
    tempfile::NamedTempFile,
};

pub struct FsRepo;

impl FsRepoOps for FsRepo {
    fn check_existing_dir(&self, path: &Path) -> Result<()> {
        super::check_existing_dir(path)
    }

    fn check_existing_file(&self, path: &Path) -> Result<()> {
        super::check_existing_file(path)
    }

    fn ensure_dir(&self, path: &Path) -> Result<()> {
        super::ensure_dir(path)
    }

    fn get_file_size(&self, path: &Path) -> Result<u64> {
        super::get_file_size(path)
    }

    fn trash_file(&self, path: &Path) -> Result<()> {
        super::trash_file(path)
    }

    fn atomic_write<F>(&self, path: &Path, write_fn: F) -> Result<()>
    where
        F: FnOnce(&mut NamedTempFile) -> Result<()>,
    {
        super::atomic_write(path, write_fn)
    }
}
