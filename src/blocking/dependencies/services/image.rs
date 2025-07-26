#[cfg(test)]
use crate::blocking::traits::{MockFsRepoOps, MockHttpClientOps, MockMetadataOps};
use crate::blocking::{
    dependencies::{FsRepo, HttpClient, MetadataService},
    traits::{FsRepoOps, HttpClientOps, ImageServiceOps, MetadataOps},
};

pub struct ImageService<C, M, FS> {
    pub http: C,
    pub metadata: M,
    pub fs: FS,
}

impl<C, M, FS> ImageServiceOps for ImageService<C, M, FS>
where
    C: HttpClientOps,
    M: MetadataOps,
    FS: FsRepoOps,
{
    type HttpClient = C;
    type Metadata = M;
    type FsRepo = FS;

    fn http(&self) -> &C {
        &self.http
    }

    fn metadata(&self) -> &M {
        &self.metadata
    }

    fn fs(&self) -> &FS {
        &self.fs
    }
}

impl Default for ImageService<HttpClient, MetadataService, FsRepo> {
    fn default() -> Self {
        Self { http: HttpClient, metadata: MetadataService, fs: FsRepo }
    }
}

#[cfg(test)]
pub struct MockImageService {
    pub fs: MockFsRepoOps,
    pub metadata: MockMetadataOps,
    pub http: MockHttpClientOps,
}

#[cfg(test)]
impl ImageServiceOps for MockImageService {
    type HttpClient = MockHttpClientOps;
    type Metadata = MockMetadataOps;
    type FsRepo = MockFsRepoOps;

    fn http(&self) -> &Self::HttpClient {
        &self.http
    }

    fn metadata(&self) -> &Self::Metadata {
        &self.metadata
    }

    fn fs(&self) -> &Self::FsRepo {
        &self.fs
    }
}

#[cfg(test)]
impl Default for MockImageService {
    fn default() -> Self {
        Self {
            http: MockHttpClientOps::new(),
            fs: MockFsRepoOps::new(),
            metadata: MockMetadataOps::new(),
        }
    }
}
