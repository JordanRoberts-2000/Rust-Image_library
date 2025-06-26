use base64::{engine::general_purpose::STANDARD as BASE64, Engine};

use crate::{Image, Result};

impl Image {
    pub fn to_data_url(&mut self) -> Result<String> {
        self.apply_transforms()?;

        let mut buffer = Vec::new();
        self.encode(&mut buffer)?;

        let base64 = BASE64.encode(buffer);
        let data_url = format!("data:{};base64,{}", self.format.to_mime_type(), base64);

        Ok(data_url)
    }
}
