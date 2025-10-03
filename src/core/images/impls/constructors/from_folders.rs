use {
    crate::{FromFolderConfig, Images, Result},
    std::path::Path,
};

impl Images {
    pub fn from_folders<I, P>(paths: I) -> Result<Self>
    where
        I: IntoIterator<Item = P>,
        P: AsRef<Path>,
    {
        Self::from_folders_with_config(paths, FromFolderConfig::default())
    }

    pub fn from_folders_with_config<I, P>(paths: I, config: FromFolderConfig) -> Result<Self>
    where
        I: IntoIterator<Item = P>,
        P: AsRef<Path>,
    {
        let mut all_images = Vec::new();

        for path in paths {
            let path_ref = path.as_ref();

            let mut images = Self::from_folder_with_config(path_ref, &config)?;
            all_images.append(&mut images);
        }

        Ok(Images::from_vec(all_images))
    }
}
