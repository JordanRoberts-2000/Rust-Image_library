use {
    crate::{
        blocking::Image,
        encoders::{AvifEncoder, JpegEncoder, PngEncoder, WebPEncoder},
        CompressionType, ImageFormat, Result,
    },
    std::io::Write,
};

impl Image {
    pub fn encode(&mut self, writer: impl Write, format: ImageFormat) -> Result<()> {
        match format {
            ImageFormat::Jpeg => {
                let (quality, progressive) = self.config.resolve_jpeg_config();
                let (width, height) = (self.width(), self.height());

                let img = self.process_image()?;

                let encoder = JpegEncoder::new()
                    .with_quality(quality)
                    .with_color_type(img.as_ref().color().into())
                    .set_progressive(progressive);

                encoder.from_raw_pixels(img.as_ref().as_bytes(), width, height).write_to(writer)?;
            }
            ImageFormat::Png => {
                let img = self.process_image()?;

                PngEncoder::new()
                    .with_color_type(img.as_ref().color().into())
                    .write_to(img.as_ref().as_bytes(), writer)?;
            }
            ImageFormat::WebP => {
                let (compression, quality) = self.config.resolve_webp_config();
                let img = self.process_image()?;

                WebPEncoder::new()
                    .with_compression(compression)
                    .with_quality(quality)
                    .with_color_type(img.as_ref().color().into())
                    .write_to(img.as_ref().as_bytes(), writer)?;
            }
            ImageFormat::Avif => {
                if self.config.compression == CompressionType::Lossless {
                    log::warn!(
                        "Lossless AVIF compression is not supported; falling back to lossy."
                    );
                }
                let (quality, speed, alpha_quality) = self.config.resolve_avif_config();
                let img = self.process_image()?;

                AvifEncoder::new()
                    .with_alpha_quality(alpha_quality)
                    .with_color_type(img.as_ref().color().into())
                    .with_speed(speed)
                    .with_quality(quality)
                    .write_to(img.as_ref().as_bytes(), writer)?;
            }
        };

        Ok(())
    }
}
