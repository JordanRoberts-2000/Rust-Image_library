use std::path::PathBuf;

#[derive(Debug)]
pub enum ImageSrc {
    File { path: PathBuf },
    Url { url: url::Url },
    Bytes,
    Reader,
}
