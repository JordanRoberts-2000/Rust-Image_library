use crate::blocking::traits::{FsRepoOps, HttpClientOps, IoRepoOps, MetadataOps};

pub trait ImageServiceOps {
    type HttpClient: HttpClientOps;
    type Metadata: MetadataOps;
    type FsRepo: FsRepoOps;
    type IoRepo: IoRepoOps;

    fn http(&self) -> &Self::HttpClient;
    fn metadata(&self) -> &Self::Metadata;
    fn fs(&self) -> &Self::FsRepo;
    fn io(&self) -> &Self::IoRepo;
}
