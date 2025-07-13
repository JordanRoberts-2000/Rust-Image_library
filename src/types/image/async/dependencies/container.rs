use crate::image::r#async::{
    dependencies::{FsRepo, Metadata, UrlDownloader},
    traits::{FsRepoOps, ImageDepsOps, MetadataOps, UrlDownloaderOp},
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
    FS: FsRepoOps,
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
