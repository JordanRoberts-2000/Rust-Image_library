use {
    image::{codecs::png::PngEncoder, ColorType, ImageEncoder},
    std::io::Write,
};

use crate::{Image, ImageError, ImageFormat, Result};

impl Image {
    pub fn encode_png(&mut self, writer: impl Write) -> Result<()> {
        let rgb8 = self.get_decoded()?.to_rgb8();
        let (width, height) = rgb8.dimensions();

        let encoder = PngEncoder::new(writer);
        encoder
            .write_image(&rgb8, width, height, ColorType::Rgb8.into())
            .map_err(|e| ImageError::Encoding {
                source: e,
                id: self.describe_source(),
                format: ImageFormat::Png,
            })?;

        Ok(())
    }
}
