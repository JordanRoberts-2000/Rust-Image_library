use {
    crate::{
        archive_formats::{Tar, TarGz, Zip},
        Archive, Images, Result,
    },
    fs_ext::file,
    std::path::Path,
};

impl Images {
    pub fn to_archive<A: Archive>(&mut self, path: impl AsRef<Path>) -> Result<()> {
        // self.handle_file_name_collisions()?;

        file::atomic::overwrite(path, |file| A::write_images(file, &mut self.inner))?;

        Ok(())
    }

    pub fn to_zip(&mut self, path: impl AsRef<Path>) -> Result<()> {
        self.to_archive::<Zip>(path)
    }

    pub fn to_tar(&mut self, path: impl AsRef<Path>) -> Result<()> {
        self.to_archive::<Tar>(path)
    }

    pub fn to_tar_gz(&mut self, path: impl AsRef<Path>) -> Result<()> {
        self.to_archive::<TarGz>(path)
    }
}
