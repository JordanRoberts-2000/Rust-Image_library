use std::path::PathBuf;

#[derive(Debug)]
pub enum ImageSrc {
    File { path: PathBuf },
    Url { url: url::Url, bytes: Vec<u8> },
    Bytes(Vec<u8>),
    Reader,
}
