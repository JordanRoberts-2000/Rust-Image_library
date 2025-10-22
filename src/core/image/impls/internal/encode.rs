use {
    crate::{
        encoding::{
            AvifColorType, AvifEncoder, ColorType, GifEncoder, JpegColorType, JpegEncoder,
            PngColorType, PngEncoder, TiffColorType, TiffEncoder, WebpColorType, WebpEncoder,
        },
        image::Decoded,
        DynamicImageExt, EncodeFormat, Image, Result, ValidationError,
    },
    std::io::Write,
};

impl Image {
    pub(crate) fn encode(&self, writer: impl Write, format: EncodeFormat) -> Result<()> {
        let decoded = self.decoded();
        let (w, h) = decoded.dimensions();
        let ct = ColorType::try_from(decoded.color())
            .map_err(|_| ValidationError::UnsupportedColorType(decoded.color()))?;

        match format {
            EncodeFormat::Jpeg => {
                let ct = JpegColorType::from_color_type_lossy(ct);
                let bytes = ColorType::from(ct).bytes(&decoded);
                JpegEncoder::from(self.config.jpeg()).encode(writer, bytes, w, h, ct)
            }
            EncodeFormat::Png => {
                let ct = PngColorType::from_color_type_lossy(ct);
                let bytes = ColorType::from(ct).bytes(&decoded);
                PngEncoder::from(self.config.png()).encode(writer, bytes, w, h, ct)
            }
            EncodeFormat::Webp => {
                let ct = WebpColorType::from_color_type_lossy(ct);
                let bytes = ColorType::from(ct).bytes(&decoded);
                WebpEncoder::from(self.config.webp()).encode(writer, bytes, w, h, ct)
            }
            EncodeFormat::Avif => {
                let ct = AvifColorType::from_color_type_lossy(ct);
                let bytes = ColorType::from(ct).bytes(&decoded);
                AvifEncoder::from(self.config.avif()).encode(writer, bytes, w, h, ct)
            }
            EncodeFormat::Tiff => {
                let ct = TiffColorType::from_color_type_lossy(ct);
                let bytes = ColorType::from(ct).bytes(&decoded);
                TiffEncoder.encode(writer, bytes, w, h, ct)
            }
            EncodeFormat::Gif => {
                let encoder = GifEncoder::new();
                match &*decoded {
                    Decoded::Static(img) => encoder.encode(writer, [img.clone().into_frame()]),
                    Decoded::Animated { frames, .. } => encoder.encode(writer, frames.clone()),
                }
            }
        };

        Ok(())
    }
}
