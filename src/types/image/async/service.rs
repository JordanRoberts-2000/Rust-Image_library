use crate::{
    AsyncMetadataRepo, AsyncValidationRepo, DefaultAsyncMetadataRepo, DefaultAsyncValidationRepo,
};

pub struct AsyncImageService<M, V>
where
    M: AsyncMetadataRepo,
    V: AsyncValidationRepo,
{
    pub metadata: M,
    pub validation: V,
}

impl AsyncImageService<DefaultAsyncMetadataRepo, DefaultAsyncValidationRepo> {
    pub fn new() -> Self {
        Self {
            metadata: DefaultAsyncMetadataRepo::new(),
            validation: DefaultAsyncValidationRepo::new(),
        }
    }
}
