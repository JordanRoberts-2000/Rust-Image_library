use {
    crate::{Image, ImageFormat, Result},
    std::io::Write,
};

impl Image {
    pub fn encode(&mut self, writer: impl Write, format: ImageFormat) -> Result<()> {
        let bytes = self.processed_image()?.as_bytes();

        // match format {
        //     ImageFormat::Jpeg => self.jpeg_encode(writer, bytes),
        //     _ => todo!(),
        // };

        Ok(())
    }
}
