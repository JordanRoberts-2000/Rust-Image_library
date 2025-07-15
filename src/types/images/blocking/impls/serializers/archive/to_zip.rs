use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use zip::{write::FileOptions, CompressionMethod, ZipWriter};

use crate::{blocking::Images, IoError, Result};

impl Images {
    pub(crate) fn to_zip(&mut self, path: &Path) -> Result<()> {
        let file = File::create(path).map_err(|e| IoError::WriteFile(e, path.to_path_buf()))?;
        let mut zip = ZipWriter::new(BufWriter::new(file));

        let options: FileOptions<()> =
            FileOptions::default().compression_method(CompressionMethod::Stored);

        for image in self.inner.iter_mut() {
            zip.start_file(image.file_name(), options)
                .map_err(|e| IoError::ZipStartFile(e, image.file_name()))?;
            zip.write_all(&mut image.to_bytes()?)
                .map_err(|e| IoError::ZipWriteFile(e, image.file_name()))?;
        }

        zip.finish().map_err(IoError::ZipFinalize)?;
        Ok(())
    }
}
