#[derive(Debug, serde::Deserialize, Clone, Default)]
#[serde(rename_all = "lowercase")]
pub enum CompressionType {
    #[default]
    Lossy,
    Lossless,
}
