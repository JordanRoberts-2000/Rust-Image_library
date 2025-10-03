use {
    crate::{FromFolderConfig, Images, Result},
    std::path::{Path, PathBuf},
};

pub struct ImagesBuilder {
    folders: Vec<PathBuf>,
    files: Vec<PathBuf>,
    config: FromFolderConfig,
}

impl ImagesBuilder {
    pub fn new() -> Self {
        Self { folders: Vec::new(), files: Vec::new(), config: FromFolderConfig::default() }
    }

    pub fn add_folder(mut self, path: impl AsRef<Path>) -> Self {
        self.folders.push(path.as_ref().to_path_buf());
        self
    }

    pub fn add_folders<I, P>(mut self, paths: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: AsRef<Path>,
    {
        self.folders.extend(paths.into_iter().map(|p| p.as_ref().to_path_buf()));
        self
    }

    pub fn add_file(mut self, path: impl AsRef<Path>) -> Self {
        self.files.push(path.as_ref().to_path_buf());
        self
    }

    pub fn add_files<I, P>(mut self, paths: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: AsRef<Path>,
    {
        self.files.extend(paths.into_iter().map(|p| p.as_ref().to_path_buf()));
        self
    }

    pub fn config(mut self, config: FromFolderConfig) -> Self {
        self.config = config;
        self
    }

    pub fn recursive(mut self, recursive: bool) -> Self {
        self.config.recursive = recursive;
        self
    }

    pub fn build(self) -> Result<Images> {
        let mut all_images = Vec::new();

        if !self.folders.is_empty() {
            let folder_images = Images::from_folders_with_config(self.folders, self.config)?;
            all_images.extend(folder_images.inner);
        }

        if !self.files.is_empty() {
            let file_images = Images::from_files(self.files)?;
            all_images.extend(file_images.inner);
        }

        Ok(Images::from_vec(all_images))
    }
}
