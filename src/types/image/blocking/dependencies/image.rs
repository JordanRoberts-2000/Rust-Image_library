use crate::image::blocking::{
    dependencies::{Metadata, UrlDownloader},
    traits::{FsOps, ImageDepsOps, MetadataOps, UrlDownloaderOp},
    FsRepo,
};

pub struct ImageDeps<D, M, FS> {
    pub downloader: D,
    pub metadata: M,
    pub fs: FS,
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

impl<D, M, FS> ImageDepsOps for ImageDeps<D, M, FS>
where
    D: UrlDownloaderOp,
    M: MetadataOps,
    FS: FsOps,
{
    type Downloader = D;
    type Metadata = M;
    type FsRepo = FS;

    fn download(&self) -> &D {
        &self.downloader
    }

    fn metadata(&self) -> &M {
        &self.metadata
    }

    fn fs(&self) -> &FS {
        &self.fs
    }
}
