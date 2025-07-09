use base64::Engine;

use crate::{
    BlockingImage, ImageConfig, ImageData, ImageError, ImageSrc, Result, SyncImageService,
    SyncMetadataRepo, SyncValidationRepo,
};

impl BlockingImage {
    pub fn from_base64(base_64: impl AsRef<str>) -> Result<Self> {
        let base_64 = base_64.as_ref();
        Self::from_base64_internal(base_64, SyncImageService::new())
    }

    fn from_base64_internal<M: SyncMetadataRepo, V: SyncValidationRepo>(
        base_64: &str,
        service: SyncImageService<M, V>,
    ) -> Result<Self> {
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(base_64)
            .map_err(|e| ImageError::Base64DecodeFailed(e, base_64.to_string()))?;

        let (format, width, height) = service.metadata.from_bytes(&bytes)?;

        Ok(Self {
            src: ImageSrc::Base64(base_64.to_string()),
            data: ImageData::EncodedBytes(bytes),
            config: ImageConfig::default(),
            height,
            width,
            aspect_ratio: width as f32 / height as f32,
            format,
        })
    }
}
