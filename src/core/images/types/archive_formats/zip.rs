use {
    crate::{Archive, Image},
    std::io::{self, BufWriter, Seek, Write},
    zip::{write::FileOptions, CompressionMethod, ZipWriter},
};

pub struct Zip;

impl Archive for Zip {
    fn write_images(writer: impl Write + Seek, images: &mut [Image]) -> io::Result<()> {
        let mut zip = ZipWriter::new(BufWriter::new(writer));
        let options: FileOptions<()> =
            FileOptions::default().compression_method(CompressionMethod::Stored);

        for image in images.iter_mut() {
            zip.start_file(image.file_name(), options)?;
            let bytes = image.to_bytes().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            zip.write_all(&bytes)?;
        }

        zip.finish()?;
        Ok(())
    }
}
