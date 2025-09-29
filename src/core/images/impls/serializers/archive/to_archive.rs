use {
    crate::{blocking::Images, ArchiveFormat, Result},
    std::path::Path,
};

impl Images {
    pub fn to_archive(&mut self, path: impl AsRef<Path>, format: ArchiveFormat) -> Result<()> {
        let path = path.as_ref();
        self.handle_file_name_collisions()?;

        match format {
            ArchiveFormat::Zip => self.to_zip(path),
            ArchiveFormat::Tar => self.to_tar(path),
            ArchiveFormat::TarGz => self.to_tar_gz(path),
        }
    }
}
