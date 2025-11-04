use {
    crate::{ImageSrc, Images, Result},
    std::path::Path,
};

impl Images {
    pub fn from_files<I, P>(paths: I) -> Result<Self>
    where
        I: IntoIterator<Item = P>,
        P: AsRef<Path>,
    {
        let mut src_vec = Vec::new();

        for path in paths {
            src_vec.push(ImageSrc::File(path.as_ref().to_path_buf()));
        }

        Ok(Self::from_src_vec(src_vec))
    }
}
