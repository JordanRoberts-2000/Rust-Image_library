use {
    crate::{
        AvifColorType, AvifEncoder, Image, JpegColorType, JpegEncoder, PngColorType, PngEncoder,
        Result, WebPColorType, WebPEncoder, WithSrc,
    },
    image::{DynamicImage, GenericImageView},
    std::{cell::Ref, io::Write},
};

impl Image {
    pub(crate) fn png_encode(&self, writer: impl Write, img: Ref<'_, DynamicImage>) -> Result<()> {
        let config = self.config.png();
        let (w, h) = img.dimensions();
        let encoder = PngEncoder::new().with_compression_type(config.compression_type);

        let color_type: PngColorType = self.resolve_color_type(&*img)?.into();
        let bytes = color_type.bytes(&*img);

        encoder.encode(writer, &bytes, w, h, color_type).with_src(self.src())
    }

    pub(crate) fn jpeg_encode(&self, writer: impl Write, img: Ref<'_, DynamicImage>) -> Result<()> {
        let config = self.config.jpeg();
        let (w, h) = img.dimensions();
        let encoder =
            JpegEncoder::new().set_progressive(config.progressive).with_quality(config.quality);

        let color_type: JpegColorType =
            self.resolve_color_type(&*img)?.try_into().with_src(self.src())?;
        let bytes = color_type.bytes(&*img);

        encoder.encode(writer, &bytes, w, h, color_type).with_src(self.src())
    }

    pub(crate) fn webp_encode(&self, writer: impl Write, img: Ref<'_, DynamicImage>) -> Result<()> {
        let config = self.config.webp();
        let (w, h) = img.dimensions();
        let encoder = if config.lossless {
            WebPEncoder::lossless()
        } else {
            WebPEncoder::lossy(config.quality)
        };

        let color_type: WebPColorType =
            self.resolve_color_type(&*img)?.try_into().with_src(self.src())?;
        let bytes = color_type.bytes(&*img);

        encoder.encode(writer, &bytes, w, h, color_type).with_src(self.src())
    }

    pub(crate) fn avif_encode(&self, writer: impl Write, img: Ref<'_, DynamicImage>) -> Result<()> {
        let config = self.config.avif();
        let (w, h) = img.dimensions();
        let encoder = AvifEncoder::new().with_speed(config.speed).with_quality(config.quality);

        let color_type: AvifColorType =
            self.resolve_color_type(&*img)?.try_into().with_src(self.src())?;
        let bytes = color_type.bytes(&*img);

        encoder.encode(writer, &bytes, w, h, color_type).with_src(self.src())
    }
}
