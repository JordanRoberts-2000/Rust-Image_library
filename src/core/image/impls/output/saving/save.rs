use {
    crate::{Image, Result, WithOrigin},
    fs_ext::file,
    std::io,
};

impl Image {
    pub fn save(&self) -> Result<()> {
        let path = self.config.output_dir.join(self.file_name());

        file::atomic::create_with(
            &path,
            |file| {
                self.encode(file, self.encoding_format())
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
            },
            self.write_options(),
        )
        .with_origin(self.origin())?;

        Ok(())
    }
}
