#[derive(Debug, serde::Deserialize, Clone, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CompressionType {
    #[default]
    Lossy,
    Lossless,
}
