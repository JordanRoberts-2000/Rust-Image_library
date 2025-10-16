use {
    crate::encoding::{Encoder, EncodingError, TiffColorType},
    std::io::Write,
};

#[derive(Debug, Clone)]
pub struct TiffEncoder;

impl TiffEncoder {
    pub fn encode(
        &self, mut writer: impl Write, bytes: impl AsRef<[u8]>, w: u32, h: u32,
        ct: impl Into<TiffColorType>,
    ) -> Result<(), EncodingError> {
        <Self as Encoder>::encode(&self, &mut writer, bytes.as_ref(), w, h, ct.into())
    }

    pub fn encode_to_vec(
        &self, bytes: impl AsRef<[u8]>, w: u32, h: u32, ct: impl Into<TiffColorType>,
    ) -> Result<Vec<u8>, EncodingError> {
        <Self as Encoder>::encode_to_vec(&self, bytes.as_ref(), w, h, ct.into())
    }
}
