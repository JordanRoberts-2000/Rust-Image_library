use {
    crate::{
        constants::{DEFAULT_AVIF_QUALITY, DEFAULT_AVIF_SPEED},
        AvifColorType, AvifEncoder as SyncAvifEncoder, EncodingError,
    },
    tokio::{
        io::{AsyncWrite, AsyncWriteExt},
        task,
    },
};

#[derive(Debug, Clone)]
pub struct AvifEncoder {
    quality: u8,
    speed: u8,
}

impl AvifEncoder {
    pub fn new() -> Self {
        Self { quality: DEFAULT_AVIF_QUALITY, speed: DEFAULT_AVIF_SPEED }
    }

    pub fn with_quality(mut self, quality: u8) -> Self {
        self.quality = quality.clamp(1, 100);
        self
    }

    pub fn with_speed(mut self, speed: u8) -> Self {
        self.speed = speed.clamp(1, 10);
        self
    }

    pub async fn encode<W>(
        &self, mut writer: W, bytes: Vec<u8>, width: u32, height: u32, color_type: AvifColorType,
    ) -> Result<(), EncodingError>
    where
        W: AsyncWrite + Unpin,
    {
        let sync_encoder = SyncAvifEncoder { quality: self.quality, speed: self.speed };

        let encoded_data = task::spawn_blocking(move || {
            let mut out = Vec::new();
            sync_encoder.encode(&mut out, &bytes, width, height, color_type)?;
            Ok::<Vec<u8>, EncodingError>(out)
        })
        .await
        .map_err(EncodingError::Join)??;

        writer.write_all(&encoded_data).await.map_err(EncodingError::Io)?;
        writer.flush().await.map_err(EncodingError::Io)?;
        Ok(())
    }
}

impl Default for AvifEncoder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use {super::*, crate::test_utils::create_rgb8_data};

    #[tokio::test]
    async fn test_encode_ok() {
        let encoder = AvifEncoder::new();
        let mut output = Vec::new();
        let (width, height) = (12, 12);
        let rgb_data = create_rgb8_data(width, height);

        let result =
            encoder.encode(&mut output, rgb_data, width, height, AvifColorType::Rgb8).await;

        assert!(result.is_ok());
        assert!(!output.is_empty(), "Output should contain encoded data");
    }
}
