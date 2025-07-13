use crate::image::blocking::traits::{FsRepoOps, MetadataOps, UrlDownloaderOp};

pub trait ImageDepsOps {
    type Downloader: UrlDownloaderOp;
    type Metadata: MetadataOps;
    type FsRepo: FsRepoOps;

    fn downloader(&self) -> &Self::Downloader;
    fn metadata(&self) -> &Self::Metadata;
    fn fs(&self) -> &Self::FsRepo;
}
