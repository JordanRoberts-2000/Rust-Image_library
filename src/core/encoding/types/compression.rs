#[derive(Debug, serde::Deserialize, Clone, Default, PartialEq, Copy, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CompressionType {
    #[default]
    Lossy,
    Lossless,
}
