use {
    crate::{ImageError, ValidationError},
    std::path::{Path, PathBuf},
    strum::IntoEnumIterator,
    strum_macros::EnumIter,
};

const SUPPORTED_EXTS: &[&str] = &["webp", "png", "jpeg", "jpg", "avif"];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, EnumIter, Hash)]
pub enum ImageFormat {
    WebP,
    #[default]
    Png,
    Jpeg,
    Avif,
}

impl ImageFormat {
    pub fn supported_exts() -> &'static [&'static str] {
        SUPPORTED_EXTS
    }

    pub fn variants() -> Vec<Self> {
        ImageFormat::iter().collect()
    }

    pub fn is_supported_ext(ext: &str) -> bool {
        let e = ext.trim_start_matches('.').to_ascii_lowercase();
        SUPPORTED_EXTS.contains(&e.as_str())
    }

    pub fn to_mime_type(&self) -> &'static str {
        match self {
            ImageFormat::WebP => "image/webp",
            ImageFormat::Png => "image/png",
            ImageFormat::Jpeg => "image/jpeg",
            ImageFormat::Avif => "image/avif",
        }
    }

    pub fn extension(&self) -> &'static str {
        match self {
            ImageFormat::WebP => "webp",
            ImageFormat::Png => "png",
            ImageFormat::Jpeg => "jpeg",
            ImageFormat::Avif => "avif",
        }
    }

    pub fn extensions(&self) -> &'static [&'static str] {
        match self {
            ImageFormat::Jpeg => &["jpeg", "jpg"],
            ImageFormat::Png => &["png"],
            ImageFormat::WebP => &["webp"],
            ImageFormat::Avif => &["avif"],
        }
    }
}

impl TryFrom<&str> for ImageFormat {
    type Error = ImageError;

    fn try_from(ext: &str) -> Result<Self, Self::Error> {
        match ext.to_ascii_lowercase().as_str() {
            "webp" => Ok(ImageFormat::WebP),
            "png" => Ok(ImageFormat::Png),
            "jpg" | "jpeg" => Ok(ImageFormat::Jpeg),
            "avif" => Ok(ImageFormat::Avif),
            _ => Err(ValidationError::UnsupportedExtension(ext.to_string()).into()),
        }
    }
}

impl TryFrom<String> for ImageFormat {
    type Error = ImageError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        ImageFormat::try_from(s.as_str())
    }
}

impl TryFrom<&Path> for ImageFormat {
    type Error = ImageError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        match path.extension() {
            Some(extension) => {
                let ext_str = extension
                    .to_str()
                    .ok_or_else(|| {
                        ValidationError::InvalidExtensionFormat(extension.to_os_string())
                    })?
                    .to_lowercase();

                match ext_str.as_str() {
                    "jpg" | "jpeg" => Ok(ImageFormat::Jpeg),
                    "png" => Ok(ImageFormat::Png),
                    "webp" => Ok(ImageFormat::WebP),
                    "avif" => Ok(ImageFormat::Avif),
                    _ => Err(ValidationError::UnsupportedExtension(ext_str).into()),
                }
            }
            None => Err(ValidationError::MissingExtensionForPath(path.to_path_buf()).into()),
        }
    }
}

impl TryFrom<PathBuf> for ImageFormat {
    type Error = ImageError;
    fn try_from(p: PathBuf) -> Result<Self, Self::Error> {
        ImageFormat::try_from(p.as_path())
    }
}

impl TryFrom<image::ImageFormat> for ImageFormat {
    type Error = ImageError;

    fn try_from(fmt: image::ImageFormat) -> Result<Self, Self::Error> {
        match fmt {
            image::ImageFormat::Png => Ok(ImageFormat::Png),
            image::ImageFormat::Jpeg => Ok(ImageFormat::Jpeg),
            image::ImageFormat::WebP => Ok(ImageFormat::WebP),
            image::ImageFormat::Avif => Ok(ImageFormat::Avif),
            other => Err(ValidationError::UnsupportedFormat(other).into()),
        }
    }
}

impl From<ImageFormat> for image::ImageFormat {
    fn from(fmt: ImageFormat) -> image::ImageFormat {
        match fmt {
            ImageFormat::WebP => image::ImageFormat::WebP,
            ImageFormat::Png => image::ImageFormat::Png,
            ImageFormat::Jpeg => image::ImageFormat::Jpeg,
            ImageFormat::Avif => image::ImageFormat::Avif,
        }
    }
}
