use crate::blocking::{
    dependencies::{FsRepo, HttpClient, IoRepo, MetadataService},
    traits::{FsRepoOps, HttpClientOps, ImageServiceOps, IoRepoOps, MetadataOps},
};

pub struct ImageService<C, M, FS, IO> {
    pub http: C,
    pub metadata: M,
    pub fs: FS,
    pub io: IO,
}

impl<C, M, FS, IO> ImageServiceOps for ImageService<C, M, FS, IO>
where
    C: HttpClientOps,
    M: MetadataOps,
    FS: FsRepoOps,
    IO: IoRepoOps,
{
    type HttpClient = C;
    type Metadata = M;
    type FsRepo = FS;
    type IoRepo = IO;

    fn http(&self) -> &C {
        &self.http
    }

    fn metadata(&self) -> &M {
        &self.metadata
    }

    fn fs(&self) -> &FS {
        &self.fs
    }

    fn io(&self) -> &IO {
        &self.io
    }
}

impl Default for ImageService<HttpClient, MetadataService, FsRepo, IoRepo> {
    fn default() -> Self {
        Self { http: HttpClient, metadata: MetadataService, fs: FsRepo, io: IoRepo }
    }
}
