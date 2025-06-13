use std::path::PathBuf;

#[derive(Debug)]
pub enum ImgSrc {
    File { path: PathBuf },
    Url { url: url::Url },
    Bytes,
    Reader,
}
