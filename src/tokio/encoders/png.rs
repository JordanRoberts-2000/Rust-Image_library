use {
    crate::{EncodingError, PngColorType, PngCompressionType, PngEncoder as SyncPngEncoder},
    tokio::{
        io::{AsyncWrite, AsyncWriteExt},
        task,
    },
};

#[derive(Debug, Clone)]
pub struct PngEncoder {
    compression_type: PngCompressionType,
}

impl PngEncoder {
    pub fn new() -> Self {
        Self { compression_type: PngCompressionType::Default }
    }

    pub fn best_compression() -> Self {
        Self { compression_type: PngCompressionType::Best }
    }

    pub fn fast() -> Self {
        Self { compression_type: PngCompressionType::Fast }
    }

    pub fn with_compression_type(mut self, compression_type: PngCompressionType) -> Self {
        self.compression_type = compression_type;
        self
    }

    pub async fn encode<W>(
        &self, mut writer: W, bytes: Vec<u8>, width: u32, height: u32, color_type: PngColorType,
    ) -> Result<(), EncodingError>
    where
        W: AsyncWrite + Unpin,
    {
        let sync_encoder = SyncPngEncoder::new().with_compression_type(self.compression_type);

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

impl Default for PngEncoder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use {super::*, crate::test_utils::create_rgb8_data};

    #[tokio::test]
    async fn test_encode_ok() {
        let encoder = PngEncoder::new();
        let mut output = Vec::new();
        let (width, height) = (12, 12);
        let rgb_data = create_rgb8_data(width, height);

        let result = encoder.encode(&mut output, rgb_data, width, height, PngColorType::Rgb8).await;

        assert!(result.is_ok());
        assert!(!output.is_empty(), "Output should contain encoded data");
    }
}
