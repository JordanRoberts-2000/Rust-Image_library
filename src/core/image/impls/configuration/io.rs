use {
    crate::{Image, ImageSrc, Result, WithSrc},
    fs_ext::file,
};

impl Image {
    pub fn delete_src(&self) -> Result<()> {
        if let ImageSrc::File(path) = &self.src {
            file::trash_or_remove(path).with_src(self.src())?;
        }

        Ok(())
    }
}
