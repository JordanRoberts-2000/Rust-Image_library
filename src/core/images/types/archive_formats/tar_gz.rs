use {
    crate::{Archive, Image},
    flate2::{write::GzEncoder, Compression},
    std::io::{self, BufWriter, Seek, Write},
    tar::{Builder as TarBuilder, Header},
};

pub struct TarGz;

impl Archive for TarGz {
    fn write_images(writer: impl Write + Seek, images: &mut [Image]) -> io::Result<()> {
        let gz = GzEncoder::new(BufWriter::new(writer), Compression::default());
        let mut tar = TarBuilder::new(gz);

        for image in images.iter_mut() {
            let bytes = image.to_bytes().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

            let mut header = Header::new_gnu();
            header.set_size(bytes.len() as u64);
            header.set_mode(0o644);
            header.set_cksum();

            tar.append_data(&mut header, image.file_name(), &bytes[..])?;
        }

        tar.finish()?;
        Ok(())
    }
}
