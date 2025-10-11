use {
    crate::{
        encoding::{AvifEncoder, JpegEncoder, PngEncoder, TiffEncoder, WebpEncoder},
        image::types::ResolvedColorType,
        Image, ImageFormat, Result,
    },
    image::GenericImageView,
    std::io::Write,
};

impl Image {
    pub(crate) fn encode(&self, writer: impl Write, format: ImageFormat) -> Result<()> {
        let img = self.processed_image();
        let (w, h) = img.dimensions();

        match self.resolve_color_type(&*img, format)? {
            ResolvedColorType::Jpeg(ct) => {
                JpegEncoder::from(self.config.jpeg()).encode(writer, ct.bytes(&*img), w, h, ct)
            }
            ResolvedColorType::Png(ct) => {
                PngEncoder::from(self.config.png()).encode(writer, ct.bytes(&*img), w, h, ct)
            }
            ResolvedColorType::Webp(ct) => {
                WebpEncoder::from(self.config.webp()).encode(writer, ct.bytes(&*img), w, h, ct)
            }
            ResolvedColorType::Avif(ct) => {
                AvifEncoder::from(self.config.avif()).encode(writer, ct.bytes(&*img), w, h, ct)
            }
            ResolvedColorType::Tiff(ct) => TiffEncoder::encode(writer, ct.bytes(&*img), w, h, ct),
        }?;

        Ok(())
    }
}
