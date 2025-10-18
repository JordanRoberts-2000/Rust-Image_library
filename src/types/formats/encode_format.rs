use {
    crate::{
        constants::MAGIC_BYTES_READ_LIMIT,
        format_detection::{Guessable, Guesser},
        utils::normalise_ext,
        EncodeFormat, Format, FormatOps, ImageError, ValidationError,
    },
    inherent::inherent,
    std::{
        fmt,
        io::BufRead,
        path::{Path, PathBuf},
        str,
    },
};

impl Guessable for EncodeFormat {
    fn guess(&self, bytes: &[u8]) -> bool {
        Format::from(*self).guess(bytes)
    }

    fn ext_guess(&self, ext: &str) -> bool {
        Format::from(*self).ext_guess(ext)
    }

    fn read_limit(&self) -> usize {
        MAGIC_BYTES_READ_LIMIT
    }
}

impl EncodeFormat {
    pub fn guesser() -> Guesser<Self> {
        <Self as Guessable>::guesser()
    }
    pub fn guess_from_file(path: impl AsRef<Path>) -> Result<Option<Self>, ImageError> {
        <Self as Guessable>::guess_from_file(path)
    }
    pub fn guess_from_bytes(bytes: &[u8]) -> Option<Self> {
        <Self as Guessable>::guess_from_bytes(bytes)
    }
    pub fn guess_from_reader<R: BufRead>(r: &mut R) -> Result<Option<Self>, ImageError> {
        <Self as Guessable>::guess_from_reader(r)
    }
}

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
            "webp" => EncodeFormat::Webp,
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
            I::WebP => Ok(EncodeFormat::Webp),
            I::Png => Ok(EncodeFormat::Png),
            I::Jpeg => Ok(EncodeFormat::Jpeg),
            I::Avif => Ok(EncodeFormat::Avif),
            I::Tiff => Ok(EncodeFormat::Tiff),
            other => Err(ValidationError::UnsupportedImageFormat(other).into()),
        }
    }
}
