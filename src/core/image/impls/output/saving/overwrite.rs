use {
    crate::{Image, Result, WithSrc},
    fs_ext::file,
};

impl Image {
    pub fn overwrite(&self) -> Result<()> {
        let path = self.config.output_dir.join(self.file_name());

        file::atomic::overwrite(&path, |file| self.encode(file, self.encoding_format()))
            .with_src(self.src())?;

        Ok(())
    }
}
