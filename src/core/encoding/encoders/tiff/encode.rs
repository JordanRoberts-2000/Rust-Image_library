use {
    crate::{
        encoding::{EncoderOps, EncodingErrorKind, TiffColorType, TiffEncoder},
        ImageFormat,
    },
    image::codecs::tiff::TiffEncoder as ImageEncoder,
    std::io::{Cursor, Write},
};

impl EncoderOps for TiffEncoder {
    type ColorType = TiffColorType;
    const IMAGE_FORMAT: ImageFormat = ImageFormat::Tiff;

    fn encode_impl(
        &self, mut writer: impl Write, bytes: &[u8], w: u32, h: u32, ct: Self::ColorType,
    ) -> Result<(), EncodingErrorKind> {
        let mut buffer = Vec::new();
        ImageEncoder::new(Cursor::new(&mut buffer))
            .encode(bytes, w, h, ct.into())
            .map_err(|e| EncodingErrorKind::Encode(Box::new(e)))?;

        writer.write_all(&buffer)?;

        Ok(())
    }
}
