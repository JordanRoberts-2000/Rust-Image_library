use {
    crate::encoding::{EncoderOps, EncodingError, TiffColorType},
    std::io::Write,
};

#[derive(Debug, Clone)]
pub struct TiffEncoder;

impl TiffEncoder {
    pub fn encode(
        writer: impl Write, bytes: impl AsRef<[u8]>, w: u32, h: u32, ct: impl Into<TiffColorType>,
    ) -> Result<(), EncodingError> {
        <Self as EncoderOps>::encode(&TiffEncoder, writer, bytes, w, h, ct)
    }
}
