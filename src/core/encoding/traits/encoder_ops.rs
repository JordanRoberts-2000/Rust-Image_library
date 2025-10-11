use {
    crate::{
        encoding::{utils, ColorType, EncodingError, EncodingErrorKind},
        ImageFormat,
    },
    std::io::Write,
};

pub trait EncoderOps {
    type ColorType: Copy + Into<ColorType>;
    const IMAGE_FORMAT: ImageFormat;

    fn encode_impl(
        &self, writer: impl Write, bytes: &[u8], w: u32, h: u32, ct: Self::ColorType,
    ) -> Result<(), EncodingErrorKind>;

    fn encode(
        &self, writer: impl Write, bytes: impl AsRef<[u8]>, w: u32, h: u32,
        color_type: impl Into<Self::ColorType>,
    ) -> Result<(), EncodingError> {
        let ct = color_type.into();
        Self::validate_buffer(bytes.as_ref(), w, h, ct)
            .map_err(|kind| EncodingError::new(Self::IMAGE_FORMAT, kind))?;

        self.encode_impl(writer, bytes.as_ref(), w, h, ct)
            .map_err(|kind| EncodingError::new(Self::IMAGE_FORMAT, kind))
    }

    fn validate_buffer(
        bytes: &[u8], w: u32, h: u32, ct: Self::ColorType,
    ) -> Result<(), EncodingErrorKind> {
        utils::validate_buffer(bytes, w, h, ct)
    }
}
