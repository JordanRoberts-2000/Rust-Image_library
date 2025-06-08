use std::path::PathBuf;

#[derive(Debug, serde::Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum CompressionType {
    Lossy,
    Lossless,
}

#[derive(Debug)]
pub enum ImgSrc {
    Local { path: PathBuf },
    Remote { url: url::Url },
    Bytes { id: String },
}

pub enum ValidImageFormat {
    Webp,
    Png,
    Jpeg,
}

impl TryFrom<&str> for ValidImageFormat {
    type Error = ();

    fn try_from(ext: &str) -> Result<Self, Self::Error> {
        match ext.to_ascii_lowercase().as_str() {
            "webp" => Ok(ValidImageFormat::Webp),
            "png" => Ok(ValidImageFormat::Png),
            "jpg" | "jpeg" => Ok(ValidImageFormat::Jpeg),
            _ => Err(()),
        }
    }
}
