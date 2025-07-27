use {
    crate::{blocking::Image, ImageFormat, Result},
    std::io::Write,
};

impl Image {
    pub fn encode(&mut self, writer: impl Write, format: ImageFormat) -> Result<()> {
        let (width, height) = (self.width(), self.height());

        Ok(())
        // match format {
        //     ImageFormat::Jpeg => {
        //         let encoder = &self.config.create_jpeg_encoder();
        //         let img = self.process_image()?;

        //         encoder.from_raw_pixels(img.as_ref().as_bytes(), width, height).write_to(writer)
        //     }
        //     ImageFormat::Png => {
        //         let encoder = &self.config.create_png_encoder();
        //         let img = self.process_image()?;

        //         encoder.from_raw_pixels(img.as_ref().as_bytes(), width, height).write_to(writer)
        //     }
        //     ImageFormat::WebP => {
        //         let encoder = &self.config.create_webp_encoder();
        //         let img = self.process_image()?;

        //         encoder.from_raw_pixels(img.as_ref().as_bytes(), width, height).write_to(writer)
        //     }
        //     ImageFormat::Avif => {
        //         let encoder = &self.config.create_png_encoder();
        //         let img = self.process_image()?;

        //         encoder.from_raw_pixels(img.as_ref().as_bytes(), width, height).write_to(writer)
        //     }
        // }
    }
}
