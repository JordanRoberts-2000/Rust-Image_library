use {
    crate::{images::try_load_image, FromFolderConfig, Images, Result},
    fs_ext::{dir, DirQuery},
    std::path::Path,
};

impl Images {
    pub fn from_folder(path: impl AsRef<Path>) -> Result<Self> {
        Self::from_folder_with_config(path, &FromFolderConfig::default())
    }

    pub fn from_folder_with_config(
        path: impl AsRef<Path>, config: &FromFolderConfig,
    ) -> Result<Self> {
        dir::assert_exists(&path)?;

        let skip_errors = config.skip_errors;
        let image_paths = DirQuery::from_options(path, config.into()).collect()?;

        let mut images = Vec::new();
        for image_path in &image_paths {
            if let Some(img) = try_load_image(image_path, skip_errors)? {
                images.push(img);
            }
        }

        Ok(Self::from_vec(images))
    }
}
