use std::{fs::File, io::BufWriter, path::Path};

use crate::{Image, ImageError, ImageFormat, IoError, Result};

use image::{codecs::jpeg::JpegEncoder, ExtendedColorType};

impl Image {
    pub fn encode_jpeg(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();

        let rgb8 = self.raw.to_rgb8();
        let (width, height) = rgb8.dimensions();

        let file = File::create(path).map_err(|e| IoError::CreateFile(e, path.to_path_buf()))?;
        let writer = &mut BufWriter::new(file);

        let mut encoder = match self.config.quality {
            None => JpegEncoder::new(writer),
            Some(q) => JpegEncoder::new_with_quality(writer, q as u8),
        };

        encoder
            .encode(rgb8.as_raw(), width, height, ExtendedColorType::Rgb8)
            .map_err(|e| ImageError::Encoding {
                source: e,
                id: self.describe_source(),
                format: ImageFormat::Jpeg,
            })?;

        Ok(())
    }
}
