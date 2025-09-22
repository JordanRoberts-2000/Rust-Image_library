use {
    crate::{
        constants::DEFAULT_JPEG_QUALITY, EncodingError, JpegColorType,
        JpegEncoder as SyncJpegEncoder,
    },
    tokio::{
        io::{AsyncWrite, AsyncWriteExt},
        task,
    },
};

#[derive(Debug, Clone)]
pub struct JpegEncoder {
    quality: u8,
    progressive: bool,
}

impl JpegEncoder {
    pub fn new() -> Self {
        Self { progressive: false, quality: DEFAULT_JPEG_QUALITY }
    }

    pub fn progressive() -> Self {
        Self { progressive: true, quality: DEFAULT_JPEG_QUALITY }
    }

    pub fn with_quality(mut self, quality: u8) -> Self {
        self.quality = quality.clamp(1, 100);
        self
    }

    pub fn set_progressive(mut self, progressive: bool) -> Self {
        self.progressive = progressive;
        self
    }

    pub async fn encode<W>(
        &self, mut writer: W, bytes: Vec<u8>, width: u32, height: u32, color_type: JpegColorType,
    ) -> Result<(), EncodingError>
    where
        W: AsyncWrite + Unpin,
    {
        let sync_encoder = SyncJpegEncoder { quality: self.quality, progressive: self.progressive };

        let encoded_data = task::spawn_blocking(move || {
            let mut output = Vec::new();
            sync_encoder.encode(&mut output, &bytes, width, height, color_type)?;
            Ok::<Vec<u8>, EncodingError>(output)
        })
        .await
        .map_err(EncodingError::Join)??;

        writer.write_all(&encoded_data).await.map_err(EncodingError::Io)?;
        writer.flush().await.map_err(EncodingError::Io)?;
        Ok(())
    }
}

impl Default for JpegEncoder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use {super::*, crate::test_utils::create_rgb8_data};

    #[tokio::test]
    async fn test_encode_ok() {
        let encoder = JpegEncoder::new();
        let mut output = Vec::new();
        let (width, height) = (12, 12);
        let rgb_data = create_rgb8_data(width, height);

        let result =
            encoder.encode(&mut output, rgb_data, width, height, JpegColorType::Rgb8).await;

        assert!(result.is_ok());
        assert!(!output.is_empty(), "Output should contain encoded data");
    }
}
