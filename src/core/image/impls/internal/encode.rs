use {
    crate::{
        encoding::{
            AvifColorType, AvifEncoder, ColorType, JpegColorType, JpegEncoder, PngColorType,
            PngEncoder, TiffColorType, TiffEncoder, WebpColorType, WebpEncoder,
        },
        image::Decoded,
        EncodeFormat, Image, Result, ValidationError,
    },
    image::{
        codecs::gif::{GifEncoder, Repeat},
        Delay, Frame,
    },
    std::{io::Write, time::Duration},
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
                let mut encoder = GifEncoder::new(writer);
                encoder.set_repeat(Repeat::Infinite).unwrap();
                match &*decoded {
                    Decoded::Static(img) => {
                        let rgba = img.to_rgba8();
                        let frame = Frame::from_parts(
                            rgba,
                            0,
                            0,
                            Delay::from_saturating_duration(Duration::from_millis(0)),
                        );
                        encoder.encode_frame(frame);
                        Ok(())
                    }

                    Decoded::Animated { frames, .. } => {
                        let (head, tail) = (frames.head.clone(), frames.tail.clone());

                        encoder.encode_frame(head).unwrap();
                        for fr in tail {
                            encoder.encode_frame(fr).unwrap();
                        }
                        Ok(())
                    }
                }
            }
        };

        Ok(())
    }
}
