use {
    crate::{Image, Result, WithSrc},
    fs_ext::file,
    std::{io, path::Path},
};

impl Image {
    pub fn save_to_folder(&self, folder_path: impl AsRef<Path>) -> Result<()> {
        let path = folder_path.as_ref().join(self.file_name());

        file::atomic::create_with(
            &path,
            |file| {
                self.encode(file, self.encoding_format())
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
            },
            self.write_options(),
        )
        .with_src(self.src())?;

        Ok(())
    }
}
