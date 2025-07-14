use {
    crate::{image::r#async::ImageData, Image, ImageError, InternalError},
    image::ImageReader,
    std::sync::Arc,
    tokio::task::spawn_blocking,
};

impl Image {
    pub async fn decode(&self) -> Result<(), ImageError> {
        {
            let state = self.state.read().await;
            if matches!(state.data, ImageData::Decoded(_)) {
                return Ok(());
            }
        }

        let mut state = self.state.write().await;

        // Double-check the state to avoid race condition
        if matches!(state.data, ImageData::Decoded(_)) {
            return Ok(());
        }

        let data = match &state.data {
            ImageData::File(path) => {
                let path = path.clone();
                spawn_blocking(move || {
                    let reader = ImageReader::open(&path).map_err(|e| ImageError::Open {
                        source: e,
                        path: path.clone(),
                    })?;

                    reader.decode().map_err(|e| ImageError::DecodeFile {
                        source: e,
                        path: path.clone(),
                    })
                })
                .await
                .map_err(ImageError::TaskJoinError)?
            }
            ImageData::EncodedBytes(bytes) => {
                let bytes = Arc::clone(bytes);
                let format = state.format;
                let id = self.describe_source();

                spawn_blocking(move || {
                    image::load_from_memory_with_format(&bytes, format.into()).map_err(|e| {
                        ImageError::Decoding {
                            id,
                            format,
                            source: e,
                        }
                    })
                })
                .await
                .map_err(ImageError::TaskJoinError)?
            }
            ImageData::Decoded(_) => {
                return Err(InternalError::DecodingInvariantViolatedBeforeDecodeMatch.into());
            }
        }?;

        state.data = ImageData::Decoded(data);

        Ok(())
    }
}
