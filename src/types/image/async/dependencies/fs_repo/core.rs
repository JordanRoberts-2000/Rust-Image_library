use std::{fs::File, path::Path};

use tempfile::NamedTempFile;

use crate::{image::r#async::traits::FsRepoOps, Result};

pub struct FsRepo;

impl FsRepoOps for FsRepo {
    async fn check_existing_dir(&self, path: &Path) -> Result<()> {
        super::check_existing_dir(path).await
    }

    async fn check_existing_file(&self, path: &Path) -> Result<()> {
        super::check_existing_file(path).await
    }

    async fn ensure_dir(&self, path: &Path) -> Result<()> {
        super::ensure_dir(path).await
    }

    async fn get_file_size(&self, path: &Path) -> Result<u64> {
        super::get_file_size(path).await
    }

    async fn trash_file(&self, path: &Path) -> Result<()> {
        super::trash_file(path).await
    }

    async fn create_temp_file(&self, parent: &Path) -> Result<NamedTempFile> {
        super::create_temp_file(parent).await
    }

    async fn persist_temp_file(&self, temp: NamedTempFile, path: &Path) -> Result<File> {
        super::persist_temp_file(temp, path).await
    }
}
