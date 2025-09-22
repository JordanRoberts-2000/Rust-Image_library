use {
    crate::{
        constants::DEFAULT_WEBP_QUALITY, CompressionType, EncodingError, WebPColorType,
        WebPEncoder as SyncWebPEncoder,
    },
    tokio::{
        io::{AsyncWrite, AsyncWriteExt},
        task,
    },
};

#[derive(Clone)]
pub struct WebPEncoder {
    quality: u8,
    compression_type: CompressionType,
}

impl WebPEncoder {
    pub fn new() -> Self {
        Self::lossy(DEFAULT_WEBP_QUALITY)
    }

    pub fn lossy(quality: u8) -> Self {
        Self { quality, compression_type: CompressionType::Lossy }
    }

    pub fn lossless() -> Self {
        Self { quality: DEFAULT_WEBP_QUALITY, compression_type: CompressionType::Lossless }
    }

    pub fn with_compression_type(mut self, compression_type: CompressionType) -> Self {
        self.compression_type = compression_type;
        self
    }

    pub fn with_quality(mut self, quality: u8) -> Self {
        self.quality = quality.clamp(1, 100);
        self
    }

    pub fn set_compression_type(&mut self, compression_type: CompressionType) {
        self.compression_type = compression_type;
    }

    pub fn set_quality(&mut self, quality: u8) {
        self.quality = quality.clamp(1, 100);
    }

    pub async fn encode<W>(
        &self, mut writer: W, bytes: Vec<u8>, width: u32, height: u32, color_type: WebPColorType,
    ) -> Result<(), EncodingError>
    where
        W: AsyncWrite + Unpin,
    {
        let sync_encoder =
            SyncWebPEncoder { quality: self.quality, compression_type: self.compression_type };

        let encoded_data = task::spawn_blocking(move || {
            let mut output = Vec::new();
            sync_encoder.encode(&mut output, &bytes, width, height, color_type)?;
            Ok::<Vec<u8>, EncodingError>(output)
        })
        .await
        .map_err(|e| EncodingError::Join(e))??;

        writer.write_all(&encoded_data).await.map_err(|e| EncodingError::Io(e))?;
        writer.flush().await.map_err(|e| EncodingError::Io(e))?;

        Ok(())
    }
}

impl Default for WebPEncoder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use {super::*, crate::test_utils::create_rgb8_data};

    #[tokio::test]
    async fn test_encode_ok() {
        let encoder = WebPEncoder::new();
        let mut output = Vec::new();
        let (width, height) = (12, 12);
        let rgb_data = create_rgb8_data(width, height);

        let result =
            encoder.encode(&mut output, rgb_data, width, height, WebPColorType::Rgb8).await;

        assert!(result.is_ok());
        assert!(!output.is_empty(), "Output should contain encoded data");
    }
}
