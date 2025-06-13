#[derive(Debug, serde::Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum CompressionType {
    Lossy,
    Lossless,
}
