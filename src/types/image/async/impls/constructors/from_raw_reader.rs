use {
    crate::{ColorType, Image, ImageError, IoError, Result},
    tokio::io::{AsyncRead, AsyncReadExt},
};

impl Image {
    pub async fn from_raw_reader_async(
        reader: impl AsyncRead + Unpin,
        width: u32,
        height: u32,
        color_type: ColorType,
    ) -> Result<Self> {
        let expected_len = width as usize * height as usize * color_type.bytes_per_pixel();
        let mut pixels = Vec::with_capacity(expected_len);

        reader
            .take(expected_len as u64)
            .read_to_end(&mut pixels)
            .await
            .map_err(IoError::ReadStream)?;

        if pixels.len() != expected_len {
            return Err(ImageError::InvalidBuffer(color_type));
        }

        Self::from_raw_pixels(pixels, width, height, color_type)
    }
}
