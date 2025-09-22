use {
    crate::{Image, Result},
    base64::{engine::general_purpose::STANDARD as BASE64, Engine},
};

impl Image {
    pub fn to_base64(&self) -> Result<String> {
        let mut buffer = Vec::new();
        self.encode(&mut buffer, self.format())?;

        Ok(BASE64.encode(buffer))
    }
}
