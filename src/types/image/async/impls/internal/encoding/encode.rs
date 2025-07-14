use crate::{Image, ImageFormat, Result};

use tokio::io::AsyncWrite;

impl Image {
    pub async fn encode<W>(&self, writer: W, format: ImageFormat) -> Result<()>
    where
        W: AsyncWrite + Unpin + Send,
    {
        match format {
            ImageFormat::Jpeg => self.encode_jpeg(writer).await,
            ImageFormat::Png => self.encode_png(writer).await,
            ImageFormat::WebP => self.encode_webp(writer).await,
            ImageFormat::Avif => self.encode_avif(writer).await,
        }
    }
}
