use crate::{Image, Result};

impl Image {
    pub async fn to_data_url(&mut self) -> Result<String> {
        let base64 = self.to_base64().await?;
        let format = self.state.read().await.format;
        let data_url = format!("data:{};base64,{}", format.to_mime_type(), base64);

        Ok(data_url)
    }
}
