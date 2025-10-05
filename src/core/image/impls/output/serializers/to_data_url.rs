use crate::{Image, Result};

impl Image {
    pub fn to_data_url(&self) -> Result<String> {
        let base64 = self.to_base64()?;
        let data_url = format!("data:{};base64,{}", self.format().mime_type(), base64);

        Ok(data_url)
    }
}
