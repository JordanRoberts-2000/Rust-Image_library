use {
    crate::{image::ImageOrigin, Image, Result, WithOrigin},
    fs_ext::file,
};

impl Image {
    pub fn delete_src(&self) -> Result<()> {
        if let ImageOrigin::File(path) = &self.origin {
            file::trash_or_remove(path).with_origin(self.origin())?;
        }

        Ok(())
    }
}
