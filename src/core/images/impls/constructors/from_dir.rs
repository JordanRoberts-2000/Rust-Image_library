use {
    crate::{FromDirConfig, ImageSrc, Images, Result},
    fs_ext::DirQuery,
    std::path::Path,
};

impl Images {
    pub fn from_dir(path: impl AsRef<Path>) -> Result<Self> {
        Self::from_dir_with_config(path, &FromDirConfig::default())
    }

    pub fn from_dir_with_config(path: impl AsRef<Path>, config: &FromDirConfig) -> Result<Self> {
        let image_paths = DirQuery::from_options(path, config.into()).collect()?;

        let mut src_vec = Vec::new();
        for image_path in image_paths {
            src_vec.push(ImageSrc::File(image_path));
        }

        Ok(Self::from_src_vec(src_vec))
    }
}
