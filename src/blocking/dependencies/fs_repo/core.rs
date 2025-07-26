use {
    crate::{blocking::traits::FsRepoOps, Result},
    std::{fs::File, path::Path},
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

    fn create_temp_file(&self, parent: &Path) -> Result<NamedTempFile> {
        super::create_temp_file(parent)
    }

    fn persist_temp_file(&self, temp: NamedTempFile, path: &Path) -> Result<File> {
        super::persist_temp_file(temp, path)
    }
}
