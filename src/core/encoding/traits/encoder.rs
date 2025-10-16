use {
    crate::{
        encoding::{utils::validate_buffer, ColorType, EncodingError, EncodingErrorKind},
        ImageFormat,
    },
    std::io::Write,
};

pub trait Encoder {
    type ColorType: Copy + Into<ColorType>;
    const IMAGE_FORMAT: ImageFormat;

    fn encode_impl(
        &self, writer: &mut dyn Write, bytes: &[u8], w: u32, h: u32, ct: Self::ColorType,
    ) -> Result<(), EncodingErrorKind>;

    fn encode(
        &self, writer: &mut dyn Write, bytes: &[u8], w: u32, h: u32, ct: Self::ColorType,
    ) -> Result<(), EncodingError> {
        validate_buffer(bytes, w, h, ct.into())
            .map_err(|kind| EncodingError::new(Self::IMAGE_FORMAT, kind))?;

        self.encode_impl(writer, bytes, w, h, ct)
            .map_err(|kind| EncodingError::new(Self::IMAGE_FORMAT, kind))
    }

    fn encode_to_vec(
        &self, bytes: &[u8], w: u32, h: u32, color_type: Self::ColorType,
    ) -> Result<Vec<u8>, EncodingError> {
        let mut output = Vec::new();
        self.encode(&mut output, bytes, w, h, color_type)?;
        Ok(output)
    }
}
