use crate::{Image, Result};

use tokio::io::AsyncWrite;

impl Image {
    pub async fn to_writer<W>(&self, writer: W) -> Result<()>
    where
        W: AsyncWrite + Unpin + Send,
    {
        self.apply_transforms().await?;
        let format = self.state.read().await.format;
        self.encode(writer, format).await
    }
}
