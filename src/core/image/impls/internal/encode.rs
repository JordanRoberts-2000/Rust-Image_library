use {
    crate::{
        encoding::{
            AvifColorType, AvifEncoder, ColorType, GifEncoder, JpegColorType, JpegEncoder,
            PngColorType, PngEncoder, TiffColorType, TiffEncoder, WebpColorType, WebpEncoder,
        },
        image::Decoded,
        DynamicImageExt, EncodeFormat, Image, Result,
    },
    std::io::Write,
};

impl Image {
    pub(crate) fn encode(&self, writer: impl Write, format: EncodeFormat) -> Result<()> {
        let decoded = self.decoded();
        let (w, h) = decoded.dimensions();

        match format {
            EncodeFormat::Jpeg => {
                let ct = self.resolve_color_type::<JpegColorType>(&decoded)?;
                let bytes = ColorType::from(ct).bytes(&decoded);
                JpegEncoder::from(self.config.jpeg()).encode(writer, bytes, w, h, ct)
            }
            EncodeFormat::Png => {
                let ct = self.resolve_color_type::<PngColorType>(&decoded)?;
                let bytes = ColorType::from(ct).bytes(&decoded);
                PngEncoder::from(self.config.png()).encode(writer, bytes, w, h, ct)
            }
            EncodeFormat::Webp => {
                let ct = self.resolve_color_type::<WebpColorType>(&decoded)?;
                let bytes = ColorType::from(ct).bytes(&decoded);
                WebpEncoder::from(self.config.webp()).encode(writer, bytes, w, h, ct)
            }
            EncodeFormat::Avif => {
                let ct = self.resolve_color_type::<AvifColorType>(&decoded)?;
                let bytes = ColorType::from(ct).bytes(&decoded);
                AvifEncoder::from(self.config.avif()).encode(writer, bytes, w, h, ct)
            }
            EncodeFormat::Tiff => {
                let ct = self.resolve_color_type::<TiffColorType>(&decoded)?;
                let bytes = ColorType::from(ct).bytes(&decoded);
                TiffEncoder.encode(writer, bytes, w, h, ct)
            }
            EncodeFormat::Gif => {
                let encoder = GifEncoder::from(self.config.gif());
                match &*decoded {
                    Decoded::Static(img) => encoder.encode(writer, [img.clone().into_frame()]),
                    Decoded::Animated { frames, .. } => encoder.encode(writer, frames.clone()),
                }
            }
        };

        Ok(())
    }
}
