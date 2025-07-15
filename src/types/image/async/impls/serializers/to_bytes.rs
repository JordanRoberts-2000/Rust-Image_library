use crate::{Image, Result};

impl Image {
    pub async fn to_bytes(&mut self) -> Result<Vec<u8>> {
        self.apply_transforms().await?;
        let format = self.state.read().await.format;

        let mut buffer = Vec::new();

        self.encode(&mut buffer, format).await?;

        Ok(buffer)
    }
}
