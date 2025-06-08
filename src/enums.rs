use std::path::PathBuf;

#[derive(Debug, serde::Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum CompressionType {
    Lossy,
    Lossless,
}

pub enum ImgSrc {
    Local { path: PathBuf },
    Remote { url: url::Url },
    Bytes { id: String },
}
