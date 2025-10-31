use {
    crate::{EncodeFormat, Image, Result, ValidationError, WithSrc},
    fs_ext::file,
    std::{io, path::Path},
};

impl Image {
    pub fn save_as(&self, path: impl AsRef<Path>) -> Result<()> {
        let mut path = path.as_ref().to_path_buf();

        let ext = match path.extension() {
            None => {
                let ext = self.encoding_format().primary_extension();
                path.set_extension(ext);
                ext
            }
            Some(os_str) => os_str
                .to_str()
                .ok_or_else(|| ValidationError::MissingExtension(path.to_path_buf()))
                .with_src(self.src())?,
        };

        let format = EncodeFormat::try_from(ext).with_src(self.src())?;

        file::atomic::create_with(
            &path,
            |file| {
                self.encode(file, format)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
            },
            self.write_options(),
        )
        .with_src(self.src())?;

        Ok(())
    }
}
