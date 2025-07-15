use base64::{engine::general_purpose::STANDARD as BASE64, Engine};

use crate::{blocking::Image, Result};

impl Image {
    pub fn to_base64(&mut self) -> Result<String> {
        self.apply_transforms()?;

        let mut buffer = Vec::new();
        self.encode(&mut buffer, self.format)?;

        Ok(BASE64.encode(buffer))
    }
}
