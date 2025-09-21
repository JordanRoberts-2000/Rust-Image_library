use {
    crate::{Image, ImageFormat, Result},
    std::io::Write,
};

impl Image {
    pub fn encode(&self, writer: impl Write, format: ImageFormat) -> Result<()> {
        let img = self.processed_image()?;

        match format {
            ImageFormat::Png => self.png_encode(writer, img),
            ImageFormat::Jpeg => self.jpeg_encode(writer, img),
            ImageFormat::WebP => self.webp_encode(writer, img),
            ImageFormat::Avif => self.avif_encode(writer, img),
        }
    }
}
