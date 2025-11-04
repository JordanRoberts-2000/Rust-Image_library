use {
    crate::{FromDirConfig, Image, Images, Result},
    std::path::{Path, PathBuf},
};

pub struct ImagesBuilder {
    folders: Vec<PathBuf>,
    files: Vec<PathBuf>,
    config: FromDirConfig,
    images: Vec<Image>,
}

impl ImagesBuilder {
    pub fn new() -> Self {
        Self {
            folders: Vec::new(),
            files: Vec::new(),
            images: Vec::new(),
            config: FromDirConfig::default(),
        }
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

    pub fn add_image(mut self, image: Image) -> Self {
        self.images.push(image);
        self
    }

    pub fn add_images<I>(mut self, images: I) -> Self
    where
        I: IntoIterator<Item = Image>,
    {
        self.images.extend(images);
        self
    }

    pub fn config(mut self, config: FromDirConfig) -> Self {
        self.config = config;
        self
    }

    pub fn recursive(mut self, recursive: bool) -> Self {
        self.config.recursive = recursive;
        self
    }

    pub fn build(self) -> Result<Images> {
        let mut entry_vec = Vec::new();

        if !self.folders.is_empty() {
            for folder in self.folders {
                entry_vec.extend(Images::from_dir_with_config(folder, &self.config)?.entry_vec);
            }
        }

        if !self.files.is_empty() {
            let file_images = Images::from_files(self.files)?;
            entry_vec.extend(file_images.entry_vec);
        }

        Ok(Images { entry_vec, image_vec: self.images })
    }
}
