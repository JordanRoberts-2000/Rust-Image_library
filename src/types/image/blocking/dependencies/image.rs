use crate::image::blocking::{
    dependencies::{Metadata, UrlDownloader},
    traits::{
        FsOps, ImageDepsOps, MetadataOps, MockFsOps, MockMetadataOps, MockUrlDownloaderOp,
        UrlDownloaderOp,
    },
    FsRepo,
};

pub struct ImageDeps<D, M, FS> {
    pub downloader: D,
    pub metadata: M,
    pub fs: FS,
}

impl<D, M, FS> ImageDepsOps for ImageDeps<D, M, FS>
where
    D: UrlDownloaderOp,
    M: MetadataOps,
    FS: FsOps,
{
    type Downloader = D;
    type Metadata = M;
    type FsRepo = FS;

    fn downloader(&self) -> &D {
        &self.downloader
    }

    fn metadata(&self) -> &M {
        &self.metadata
    }

    fn fs(&self) -> &FS {
        &self.fs
    }
}

impl Default for ImageDeps<UrlDownloader, Metadata, FsRepo> {
    fn default() -> Self {
        Self {
            downloader: UrlDownloader,
            metadata: Metadata,
            fs: FsRepo,
        }
    }
}

pub struct MockImageDeps {
    pub fs: MockFsOps,
    pub metadata: MockMetadataOps,
    pub downloader: MockUrlDownloaderOp,
}

impl ImageDepsOps for MockImageDeps {
    type Downloader = MockUrlDownloaderOp;
    type Metadata = MockMetadataOps;
    type FsRepo = MockFsOps;

    fn downloader(&self) -> &Self::Downloader {
        &self.downloader
    }

    fn metadata(&self) -> &Self::Metadata {
        &self.metadata
    }

    fn fs(&self) -> &Self::FsRepo {
        &self.fs
    }
}

impl Default for MockImageDeps {
    fn default() -> Self {
        Self {
            fs: MockFsOps::new(),
            metadata: MockMetadataOps::new(),
            downloader: MockUrlDownloaderOp::new(),
        }
    }
}
