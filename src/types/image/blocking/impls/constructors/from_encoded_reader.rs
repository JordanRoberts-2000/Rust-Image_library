use std::io::{BufRead, Seek};

use crate::{
    BlockingImage, ImageConfig, ImageData, ImageSrc, IoError, Result, SyncImageService,
    SyncMetadataRepo, SyncValidationRepo,
};

impl BlockingImage {
    pub fn from_encoded_reader(mut reader: impl BufRead + Seek + 'static) -> Result<Self> {
        Self::from_encoded_reader_internal(&mut reader, SyncImageService::new())
    }

    fn from_encoded_reader_internal<R, M, V>(
        reader: &mut R,
        service: SyncImageService<M, V>,
    ) -> Result<Self>
    where
        R: BufRead + Seek + 'static,
        M: SyncMetadataRepo,
        V: SyncValidationRepo,
    {
        reader.rewind().map_err(IoError::ReadStream)?;

        let (format, width, height) = service.metadata.from_reader(reader)?;

        reader.rewind().map_err(IoError::ReadStream)?;

        let mut bytes = Vec::new();
        reader
            .read_to_end(&mut bytes)
            .map_err(IoError::ReadStream)?;

        Ok(Self {
            src: ImageSrc::Reader,
            data: ImageData::EncodedBytes(bytes),
            config: ImageConfig::default(),
            height,
            width,
            aspect_ratio: width as f32 / height as f32,
            format,
        })
    }
}
