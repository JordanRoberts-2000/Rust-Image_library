use crate::blocking::traits::{FsRepoOps, HttpClientOps, MetadataOps};

pub trait ImageServiceOps {
    type HttpClient: HttpClientOps;
    type Metadata: MetadataOps;
    type FsRepo: FsRepoOps;

    fn http(&self) -> &Self::HttpClient;
    fn metadata(&self) -> &Self::Metadata;
    fn fs(&self) -> &Self::FsRepo;
}
