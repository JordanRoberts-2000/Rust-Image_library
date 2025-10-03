use {
    crate::{Image, Images, Result},
    std::path::Path,
};

impl Images {
    pub fn from_files<I, P>(paths: I) -> Result<Self>
    where
        I: IntoIterator<Item = P>,
        P: AsRef<Path>,
    {
        let mut images = Vec::new();

        for path in paths {
            let image = Image::from_file(path)?;
            images.push(image);
        }

        Ok(Self::from_vec(images))
    }
}
