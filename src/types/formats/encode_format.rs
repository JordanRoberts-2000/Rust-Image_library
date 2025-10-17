use {
    crate::{utils::normalise_ext, EncodeFormat, Format, FormatOps, ImageError, ValidationError},
    inherent::inherent,
    std::{
        fmt,
        path::{Path, PathBuf},
        str,
    },
};

#[inherent]
impl FormatOps for EncodeFormat {
    pub fn all() -> Vec<Self>;
    pub fn supported_exts() -> Vec<&'static str>;
    pub fn is_supported_ext(ext: &str) -> bool;
    pub fn from_extension(ext: &str) -> Option<Self>;
    pub fn from_mime(mime: &str) -> Option<Self>;

    pub fn mime_type(&self) -> &'static str {
        Format::from(*self).mime_type()
    }

    pub fn primary_extension(&self) -> &'static str {
        Format::from(*self).primary_extension()
    }

    pub fn extensions(&self) -> &'static [&'static str] {
        Format::from(*self).extensions()
    }
}

impl fmt::Display for EncodeFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("{}", Format::from(*self)))
    }
}

impl TryFrom<&str> for EncodeFormat {
    type Error = ImageError;

    fn try_from(ext: &str) -> Result<Self, Self::Error> {
        let e = normalise_ext(ext);
        let enc = match e.as_str() {
            "webp" => EncodeFormat::WebP,
            "png" => EncodeFormat::Png,
            "jpg" | "jpeg" | "jpe" => EncodeFormat::Jpeg,
            "avif" => EncodeFormat::Avif,
            "tiff" | "tif" => EncodeFormat::Tiff,
            _ => return Err(ValidationError::UnsupportedExtension(ext.to_string()).into()),
        };
        Ok(enc)
    }
}

impl TryFrom<String> for EncodeFormat {
    type Error = ImageError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        EncodeFormat::try_from(s.as_str())
    }
}

impl TryFrom<&Path> for EncodeFormat {
    type Error = ImageError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        match path.extension() {
            Some(os) => {
                let ext_str = os
                    .to_str()
                    .ok_or_else(|| ValidationError::InvalidExtensionFormat(os.to_os_string()))?;
                EncodeFormat::try_from(ext_str)
            }
            None => Err(ValidationError::MissingExtensionForPath(path.to_path_buf()).into()),
        }
    }
}

impl TryFrom<PathBuf> for EncodeFormat {
    type Error = ImageError;
    fn try_from(p: PathBuf) -> Result<Self, Self::Error> {
        EncodeFormat::try_from(p.as_path())
    }
}

impl TryFrom<image::ImageFormat> for EncodeFormat {
    type Error = ImageError;

    fn try_from(fmt: image::ImageFormat) -> Result<Self, Self::Error> {
        use image::ImageFormat as I;
        match fmt {
            I::WebP => Ok(EncodeFormat::WebP),
            I::Png => Ok(EncodeFormat::Png),
            I::Jpeg => Ok(EncodeFormat::Jpeg),
            I::Avif => Ok(EncodeFormat::Avif),
            I::Tiff => Ok(EncodeFormat::Tiff),
            other => Err(ValidationError::UnsupportedImageFormat(other).into()),
        }
    }
}
