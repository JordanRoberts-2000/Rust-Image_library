use {
    crate::{utils::normalise_ext, ImageError, ValidationError},
    std::{
        fmt,
        path::{Path, PathBuf},
    },
    strum::IntoEnumIterator,
    strum_macros::EnumIter,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, EnumIter, Hash)]
pub enum ImageFormat {
    WebP,
    #[default]
    Png,
    Jpeg,
    Avif,
    Tiff,
}

impl ImageFormat {
    pub fn supported_exts() -> Vec<&'static str> {
        ImageFormat::all().iter().flat_map(|fmt| fmt.extensions()).copied().collect()
    }

    pub fn all() -> Vec<Self> {
        ImageFormat::iter().collect()
    }

    pub fn is_supported_ext(ext: &str) -> bool {
        let normalised = normalise_ext(ext);
        ImageFormat::supported_exts().contains(&normalised.as_str())
    }

    pub fn mime_type(&self) -> &'static str {
        match self {
            ImageFormat::WebP => "image/webp",
            ImageFormat::Png => "image/png",
            ImageFormat::Jpeg => "image/jpeg",
            ImageFormat::Avif => "image/avif",
            ImageFormat::Tiff => "image/tiff",
        }
    }

    pub fn extension(&self) -> &'static str {
        match self {
            ImageFormat::WebP => "webp",
            ImageFormat::Png => "png",
            ImageFormat::Jpeg => "jpeg",
            ImageFormat::Avif => "avif",
            ImageFormat::Tiff => "tiff",
        }
    }

    pub fn extensions(&self) -> &'static [&'static str] {
        match self {
            ImageFormat::Jpeg => &["jpeg", "jpg"],
            ImageFormat::Png => &["png"],
            ImageFormat::WebP => &["webp"],
            ImageFormat::Avif => &["avif"],
            ImageFormat::Tiff => &["tiff", "tif"],
        }
    }
}

impl TryFrom<&str> for ImageFormat {
    type Error = ImageError;

    fn try_from(ext: &str) -> Result<Self, Self::Error> {
        let normalised = normalise_ext(ext);
        match normalised.as_str() {
            "webp" => Ok(ImageFormat::WebP),
            "png" => Ok(ImageFormat::Png),
            "jpg" | "jpeg" => Ok(ImageFormat::Jpeg),
            "avif" => Ok(ImageFormat::Avif),
            "tiff" | "tif" => Ok(ImageFormat::Tiff),
            _ => Err(ValidationError::UnsupportedExtension(ext.to_string()).into()),
        }
    }
}

impl fmt::Display for ImageFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ImageFormat::WebP => "webp",
            ImageFormat::Png => "png",
            ImageFormat::Jpeg => "jpeg",
            ImageFormat::Avif => "avif",
            ImageFormat::Tiff => "tiff",
        };
        f.write_str(s)
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
                let ext_str = extension.to_str().ok_or_else(|| {
                    ValidationError::InvalidExtensionFormat(extension.to_os_string())
                })?;
                let normalised = normalise_ext(ext_str);

                match normalised.as_str() {
                    "jpg" | "jpeg" => Ok(ImageFormat::Jpeg),
                    "png" => Ok(ImageFormat::Png),
                    "webp" => Ok(ImageFormat::WebP),
                    "avif" => Ok(ImageFormat::Avif),
                    "tiff" | "tif" => Ok(ImageFormat::Tiff),
                    _ => Err(ValidationError::UnsupportedExtension(ext_str.to_owned()).into()),
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
            image::ImageFormat::Tiff => Ok(ImageFormat::Tiff),
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
            ImageFormat::Tiff => image::ImageFormat::Tiff,
        }
    }
}

impl From<&ImageFormat> for image::ImageFormat {
    fn from(fmt: &ImageFormat) -> image::ImageFormat {
        match fmt {
            ImageFormat::WebP => image::ImageFormat::WebP,
            ImageFormat::Png => image::ImageFormat::Png,
            ImageFormat::Jpeg => image::ImageFormat::Jpeg,
            ImageFormat::Avif => image::ImageFormat::Avif,
            ImageFormat::Tiff => image::ImageFormat::Tiff,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supported_exts() {
        let exts = ImageFormat::supported_exts();
        assert!(exts.contains(&"png"));
        assert!(exts.contains(&"jpeg"));
        assert!(exts.contains(&"jpg"));
        assert!(exts.contains(&"webp"));
        assert!(exts.contains(&"avif"));
        assert!(exts.contains(&"tiff"));
        assert!(exts.contains(&"tif"));
        assert_eq!(exts.len(), 7);
    }

    #[test]
    fn test_is_supported_ext() {
        assert!(ImageFormat::is_supported_ext("png"));
        assert!(ImageFormat::is_supported_ext("PNG"));
        assert!(ImageFormat::is_supported_ext(".png"));
        assert!(ImageFormat::is_supported_ext("jpeg"));
        assert!(ImageFormat::is_supported_ext("jpg"));
        assert!(ImageFormat::is_supported_ext("webp"));
        assert!(ImageFormat::is_supported_ext("avif"));
        assert!(ImageFormat::is_supported_ext("tiff"));
        assert!(ImageFormat::is_supported_ext(".tif"));
        assert!(!ImageFormat::is_supported_ext("gif"));
        assert!(!ImageFormat::is_supported_ext("bmp"));
    }

    #[test]
    fn test_to_mime_type() {
        assert_eq!(ImageFormat::Png.mime_type(), "image/png");
        assert_eq!(ImageFormat::Jpeg.mime_type(), "image/jpeg");
        assert_eq!(ImageFormat::WebP.mime_type(), "image/webp");
        assert_eq!(ImageFormat::Avif.mime_type(), "image/avif");
        assert_eq!(ImageFormat::Tiff.mime_type(), "image/tiff");
    }

    #[test]
    fn test_extension() {
        assert_eq!(ImageFormat::Png.extension(), "png");
        assert_eq!(ImageFormat::Jpeg.extension(), "jpeg");
        assert_eq!(ImageFormat::WebP.extension(), "webp");
        assert_eq!(ImageFormat::Avif.extension(), "avif");
        assert_eq!(ImageFormat::Tiff.extension(), "tiff");
    }

    #[test]
    fn test_extensions() {
        assert_eq!(ImageFormat::Png.extensions(), &["png"]);
        assert_eq!(ImageFormat::Jpeg.extensions(), &["jpeg", "jpg"]);
        assert_eq!(ImageFormat::WebP.extensions(), &["webp"]);
        assert_eq!(ImageFormat::Avif.extensions(), &["avif"]);
        assert_eq!(ImageFormat::Tiff.extensions(), &["tiff", "tif"]);
    }

    #[test]
    fn test_try_from_str() {
        assert_eq!(ImageFormat::try_from("png").unwrap(), ImageFormat::Png);
        assert_eq!(ImageFormat::try_from("PNG").unwrap(), ImageFormat::Png);
        assert_eq!(ImageFormat::try_from(".png").unwrap(), ImageFormat::Png);
        assert_eq!(ImageFormat::try_from("jpeg").unwrap(), ImageFormat::Jpeg);
        assert_eq!(ImageFormat::try_from("jpg").unwrap(), ImageFormat::Jpeg);
        assert_eq!(ImageFormat::try_from("webp").unwrap(), ImageFormat::WebP);
        assert_eq!(ImageFormat::try_from("avif").unwrap(), ImageFormat::Avif);
        assert_eq!(ImageFormat::try_from("tiff").unwrap(), ImageFormat::Tiff);
        assert_eq!(ImageFormat::try_from(".tif").unwrap(), ImageFormat::Tiff);
        assert!(ImageFormat::try_from("gif").is_err());
    }

    #[test]
    fn test_try_from_string() {
        assert_eq!(ImageFormat::try_from("png".to_string()).unwrap(), ImageFormat::Png);
        assert_eq!(ImageFormat::try_from("JPEG".to_string()).unwrap(), ImageFormat::Jpeg);
        assert_eq!(ImageFormat::try_from("tiff".to_string()).unwrap(), ImageFormat::Tiff);
    }

    #[test]
    fn test_try_from_path() {
        assert_eq!(ImageFormat::try_from(Path::new("image.png")).unwrap(), ImageFormat::Png);
        assert_eq!(ImageFormat::try_from(Path::new("image.PNG")).unwrap(), ImageFormat::Png);
        assert_eq!(
            ImageFormat::try_from(Path::new("/path/to/image.jpeg")).unwrap(),
            ImageFormat::Jpeg
        );
        assert_eq!(
            ImageFormat::try_from(Path::new("/path/to/image.jpg")).unwrap(),
            ImageFormat::Jpeg
        );
        assert_eq!(ImageFormat::try_from(Path::new("image.webp")).unwrap(), ImageFormat::WebP);
        assert_eq!(ImageFormat::try_from(Path::new("image.avif")).unwrap(), ImageFormat::Avif);
        assert_eq!(ImageFormat::try_from(Path::new("image.tiff")).unwrap(), ImageFormat::Tiff);
        assert_eq!(ImageFormat::try_from(Path::new("image.tif")).unwrap(), ImageFormat::Tiff);
        assert!(ImageFormat::try_from(Path::new("image.gif")).is_err());
        assert!(ImageFormat::try_from(Path::new("image")).is_err());
    }

    #[test]
    fn test_try_from_pathbuf() {
        assert_eq!(ImageFormat::try_from(PathBuf::from("image.png")).unwrap(), ImageFormat::Png);
        assert_eq!(ImageFormat::try_from(PathBuf::from("image.jpg")).unwrap(), ImageFormat::Jpeg);
        assert_eq!(ImageFormat::try_from(PathBuf::from("image.tif")).unwrap(), ImageFormat::Tiff);
    }

    #[test]
    fn test_try_from_image_format() {
        assert_eq!(ImageFormat::try_from(image::ImageFormat::Png).unwrap(), ImageFormat::Png);
        assert_eq!(ImageFormat::try_from(image::ImageFormat::Jpeg).unwrap(), ImageFormat::Jpeg);
        assert_eq!(ImageFormat::try_from(image::ImageFormat::WebP).unwrap(), ImageFormat::WebP);
        assert_eq!(ImageFormat::try_from(image::ImageFormat::Avif).unwrap(), ImageFormat::Avif);
        assert_eq!(ImageFormat::try_from(image::ImageFormat::Tiff).unwrap(), ImageFormat::Tiff);
        assert!(ImageFormat::try_from(image::ImageFormat::Gif).is_err());
    }

    #[test]
    fn test_into_image_format() {
        let fmt: image::ImageFormat = ImageFormat::Png.into();
        assert_eq!(fmt, image::ImageFormat::Png);

        let fmt: image::ImageFormat = ImageFormat::Jpeg.into();
        assert_eq!(fmt, image::ImageFormat::Jpeg);

        let fmt: image::ImageFormat = ImageFormat::WebP.into();
        assert_eq!(fmt, image::ImageFormat::WebP);

        let fmt: image::ImageFormat = ImageFormat::Avif.into();
        assert_eq!(fmt, image::ImageFormat::Avif);

        let fmt: image::ImageFormat = ImageFormat::Tiff.into();
        assert_eq!(fmt, image::ImageFormat::Tiff);
    }

    #[test]
    fn test_all() {
        let all = ImageFormat::all();
        assert_eq!(all.len(), 5);
        assert!(all.contains(&ImageFormat::Png));
        assert!(all.contains(&ImageFormat::Jpeg));
        assert!(all.contains(&ImageFormat::WebP));
        assert!(all.contains(&ImageFormat::Avif));
        assert!(all.contains(&ImageFormat::Tiff));
    }

    #[test]
    fn test_default() {
        assert_eq!(ImageFormat::default(), ImageFormat::Png);
    }
}
