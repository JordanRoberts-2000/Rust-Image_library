use {
    crate::{
        encoding::{
            AvifColorType, CompressionType, GifEncoder, JpegColorType, PngColorType, TiffColorType,
            TiffEncoder, WebpColorType,
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
                self.config.jpeg().encode(writer, ct.bytes(&decoded), w, h, ct)?;
            }
            EncodeFormat::Png => {
                let ct = self.resolve_color_type::<PngColorType>(&decoded)?;
                self.config.png().encode(writer, ct.bytes(&decoded), w, h, ct)?;
            }
            EncodeFormat::Webp => {
                let encoder = self.config.webp();
                let mut ct = self.resolve_color_type::<WebpColorType>(&decoded)?;
                if matches!(encoder.compression_type, CompressionType::Lossy) {
                    ct = ct.ensure_alpha();
                }

                self.config.webp().encode(writer, ct.bytes(&decoded), w, h, ct)?;
            }
            EncodeFormat::Avif => {
                let ct = self.resolve_color_type::<AvifColorType>(&decoded)?;
                self.config.avif().encode(writer, ct.bytes(&decoded), w, h, ct)?;
            }
            EncodeFormat::Tiff => {
                let ct = self.resolve_color_type::<TiffColorType>(&decoded)?;
                TiffEncoder.encode(writer, ct.bytes(&decoded), w, h, ct)?;
            }
            EncodeFormat::Gif => {
                let encoder = GifEncoder::from(self.config.gif());
                match &*decoded {
                    Decoded::Static(img) => encoder.encode(writer, [img.clone().into_frame()])?,
                    Decoded::Animated { frames, .. } => encoder.encode(writer, frames.clone())?,
                }
            }
        };

        Ok(())
    }
}
