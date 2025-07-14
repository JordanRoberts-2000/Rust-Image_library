use crate::{
    blocking::Image,
    image::utils::{resolve_avif_config, to_rgba8_vec},
    CompressionType, ImageError, Result,
};

use {
    ravif::{Encoder, Img},
    std::io::Write,
};

impl Image {
    pub fn encode_avif(&mut self, writer: impl Write) -> Result<()> {
        if self.config.compression == CompressionType::Lossless {
            log::warn!("Lossless AVIF compression is not supported; falling back to lossy.");
        }

        let decoded = self.get_decoded()?.to_rgba8();
        let (width, height) = decoded.dimensions();

        let (quality, speed, alpha_quality) = resolve_avif_config(&self.config);

        let pixels = to_rgba8_vec(&decoded);
        let img_ref = Img::new(pixels.as_slice(), width as usize, height as usize);

        let encoder = Encoder::new()
            .with_quality(quality)
            .with_speed(speed)
            .with_alpha_quality(alpha_quality);

        let encoded = encoder
            .encode_rgba(img_ref)
            .map_err(|err| ImageError::AvifEncoding {
                err,
                id: self.describe_source(),
            })?;

        Self::write_encoded(writer, &encoded.avif_file, self.describe_source())
    }
}
