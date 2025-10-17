use {
    crate::{utils::normalise_ext, Format, FormatOps, ImageError, ImageFormat, ValidationError},
    inherent::inherent,
    std::{
        fmt,
        path::{Path, PathBuf},
        str,
    },
};

#[inherent]
impl FormatOps for ImageFormat {
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

impl fmt::Display for ImageFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("{}", Format::from(*self)))
    }
}

impl TryFrom<&str> for ImageFormat {
    type Error = ImageError;

    fn try_from(ext: &str) -> Result<Self, Self::Error> {
        let e = normalise_ext(ext);
        let fmt = match e.as_str() {
            "png" => ImageFormat::Png,
            "jpg" | "jpeg" | "jpe" => ImageFormat::Jpeg,
            "gif" => ImageFormat::Gif,
            "webp" => ImageFormat::WebP,
            "tiff" | "tif" => ImageFormat::Tiff,
            "avif" => ImageFormat::Avif,
            "pnm" | "pbm" | "pgm" | "ppm" => ImageFormat::Pnm,
            "tga" => ImageFormat::Tga,
            "dds" => ImageFormat::Dds,
            "bmp" => ImageFormat::Bmp,
            "ico" | "cur" => ImageFormat::Ico,
            "hdr" | "pic" => ImageFormat::Hdr,
            "exr" => ImageFormat::OpenExr,
            "ff" => ImageFormat::Farbfeld,
            "qoi" => ImageFormat::Qoi,
            "pcx" => ImageFormat::Pcx,
            _ => return Err(ValidationError::UnsupportedExtension(ext.to_string()).into()),
        };
        Ok(fmt)
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
            Some(os) => {
                let ext_str = os
                    .to_str()
                    .ok_or_else(|| ValidationError::InvalidExtensionFormat(os.to_os_string()))?;
                ImageFormat::try_from(ext_str)
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
        use image::ImageFormat as I;
        match fmt {
            I::Png => Ok(ImageFormat::Png),
            I::Jpeg => Ok(ImageFormat::Jpeg),
            I::Gif => Ok(ImageFormat::Gif),
            I::WebP => Ok(ImageFormat::WebP),
            I::Tiff => Ok(ImageFormat::Tiff),
            I::Avif => Ok(ImageFormat::Avif),
            I::Pnm => Ok(ImageFormat::Pnm),
            I::Tga => Ok(ImageFormat::Tga),
            I::Dds => Ok(ImageFormat::Dds),
            I::Bmp => Ok(ImageFormat::Bmp),
            I::Ico => Ok(ImageFormat::Ico),
            I::Hdr => Ok(ImageFormat::Hdr),
            I::OpenExr => Ok(ImageFormat::OpenExr),
            I::Farbfeld => Ok(ImageFormat::Farbfeld),
            I::Qoi => Ok(ImageFormat::Qoi),
            I::Pcx => Ok(ImageFormat::Pcx),
            other => Err(ValidationError::UnsupportedImageFormat(other).into()),
        }
    }
}

impl From<ImageFormat> for image::ImageFormat {
    fn from(f: ImageFormat) -> Self {
        use image::ImageFormat as I;
        match f {
            ImageFormat::Png => I::Png,
            ImageFormat::Jpeg => I::Jpeg,
            ImageFormat::Gif => I::Gif,
            ImageFormat::WebP => I::WebP,
            ImageFormat::Tiff => I::Tiff,
            ImageFormat::Avif => I::Avif,
            ImageFormat::Pnm => I::Pnm,
            ImageFormat::Tga => I::Tga,
            ImageFormat::Dds => I::Dds,
            ImageFormat::Bmp => I::Bmp,
            ImageFormat::Ico => I::Ico,
            ImageFormat::Hdr => I::Hdr,
            ImageFormat::OpenExr => I::OpenExr,
            ImageFormat::Farbfeld => I::Farbfeld,
            ImageFormat::Qoi => I::Qoi,
            ImageFormat::Pcx => I::Pcx,
        }
    }
}
