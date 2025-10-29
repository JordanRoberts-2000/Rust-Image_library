use {
    crate::{
        constants::{MAGIC_BYTES_READ_LIMIT, TEXT_DETECTION_READ_LIMIT},
        format_detection::{detect, Guessable, Guesser},
        utils::normalise_ext,
        ErrorKind, FormatOps, ImageError, ValidationError,
    },
    inherent::inherent,
    std::{
        fmt,
        io::BufRead,
        path::{Path, PathBuf},
        str,
    },
    strum_macros::EnumIter,
    subenum::subenum,
};

#[subenum(ImageFormat, EncodeFormat)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, EnumIter, Default)]
pub enum Format {
    #[subenum(ImageFormat, EncodeFormat)]
    #[default]
    Png,
    #[subenum(ImageFormat, EncodeFormat)]
    Jpeg,
    #[subenum(ImageFormat, EncodeFormat)]
    Gif,
    #[subenum(ImageFormat, EncodeFormat)]
    Webp,
    #[subenum(ImageFormat, EncodeFormat)]
    Tiff,
    #[subenum(ImageFormat, EncodeFormat)]
    Avif,
    #[subenum(ImageFormat)]
    Pnm,
    #[subenum(ImageFormat)]
    Tga,
    #[subenum(ImageFormat)]
    Dds,
    #[subenum(ImageFormat)]
    Bmp,
    #[subenum(ImageFormat)]
    Ico,
    #[subenum(ImageFormat)]
    Hdr,
    #[subenum(ImageFormat)]
    OpenExr,
    #[subenum(ImageFormat)]
    Farbfeld,
    #[subenum(ImageFormat)]
    Qoi,
    #[subenum(ImageFormat)]
    Pcx,
    Svg,
}

impl Guessable for Format {
    fn guess(&self, bytes: &[u8]) -> bool {
        use crate::ImageFormat as IF;
        match self {
            Format::Svg => detect::svg(bytes),
            Format::Avif => IF::Avif.signatures().iter().any(|s| s.matches(bytes)),
            Format::Png => IF::Png.signatures().iter().any(|s| s.matches(bytes)),
            Format::Jpeg => IF::Jpeg.signatures().iter().any(|s| s.matches(bytes)),
            Format::Gif => IF::Gif.signatures().iter().any(|s| s.matches(bytes)),
            Format::Webp => IF::Webp.signatures().iter().any(|s| s.matches(bytes)),
            Format::Tiff => IF::Tiff.signatures().iter().any(|s| s.matches(bytes)),
            Format::Pnm => IF::Pnm.signatures().iter().any(|s| s.matches(bytes)),
            Format::Tga => IF::Tga.signatures().iter().any(|s| s.matches(bytes)),
            Format::Dds => IF::Dds.signatures().iter().any(|s| s.matches(bytes)),
            Format::Bmp => IF::Bmp.signatures().iter().any(|s| s.matches(bytes)),
            Format::Ico => IF::Ico.signatures().iter().any(|s| s.matches(bytes)),
            Format::Hdr => IF::Hdr.signatures().iter().any(|s| s.matches(bytes)),
            Format::OpenExr => IF::OpenExr.signatures().iter().any(|s| s.matches(bytes)),
            Format::Farbfeld => IF::Farbfeld.signatures().iter().any(|s| s.matches(bytes)),
            Format::Qoi => IF::Qoi.signatures().iter().any(|s| s.matches(bytes)),
            Format::Pcx => IF::Pcx.signatures().iter().any(|s| s.matches(bytes)),
        }
    }

    fn ext_guess(&self, ext: &str) -> bool {
        self.extensions().contains(&normalise_ext(ext).as_str())
    }

    fn read_limit(&self) -> usize {
        match self {
            Format::Svg => TEXT_DETECTION_READ_LIMIT,
            _ => MAGIC_BYTES_READ_LIMIT,
        }
    }
}

impl Format {
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
impl FormatOps for Format {
    pub fn all() -> Vec<Self>;
    pub fn supported_exts() -> Vec<&'static str>;
    pub fn is_supported_ext(ext: &str) -> bool;
    pub fn from_extension(ext: &str) -> Option<Self>;
    pub fn from_mime(mime: &str) -> Option<Self>;

    pub fn mime_type(&self) -> &'static str {
        match self {
            Format::Svg => "image/svg+xml",
            Format::Png => "image/png",
            Format::Jpeg => "image/jpeg",
            Format::Gif => "image/gif",
            Format::Webp => "image/webp",
            Format::Tiff => "image/tiff",
            Format::Avif => "image/avif",
            Format::Bmp => "image/bmp",
            Format::Ico => "image/vnd.microsoft.icon",
            Format::Pnm => "image/x-portable-anymap",
            Format::Tga => "image/x-tga",
            Format::Dds => "image/vnd.ms-dds",
            Format::Hdr => "image/x-radiance",
            Format::OpenExr => "image/x-exr",
            Format::Farbfeld => "image/farbfeld",
            Format::Qoi => "image/x-qoi",
            Format::Pcx => "image/x-pcx",
        }
    }

    pub fn primary_extension(&self) -> &'static str {
        match self {
            Format::Svg => "svg",
            Format::Png => "png",
            Format::Jpeg => "jpeg",
            Format::Gif => "gif",
            Format::Webp => "webp",
            Format::Tiff => "tiff",
            Format::Avif => "avif",
            Format::Pnm => "pnm",
            Format::Tga => "tga",
            Format::Dds => "dds",
            Format::Bmp => "bmp",
            Format::Ico => "ico",
            Format::Hdr => "hdr",
            Format::OpenExr => "exr",
            Format::Farbfeld => "ff",
            Format::Qoi => "qoi",
            Format::Pcx => "pcx",
        }
    }

    pub fn extensions(&self) -> &'static [&'static str] {
        match self {
            Format::Svg => &["svg"],
            Format::Png => &["png"],
            Format::Jpeg => &["jpeg", "jpg", "jpe"],
            Format::Gif => &["gif"],
            Format::Webp => &["webp"],
            Format::Tiff => &["tiff", "tif"],
            Format::Avif => &["avif"],
            Format::Pnm => &["pnm", "pbm", "pgm", "ppm"],
            Format::Tga => &["tga"],
            Format::Dds => &["dds"],
            Format::Bmp => &["bmp"],
            Format::Ico => &["ico", "cur"],
            Format::Hdr => &["hdr", "pic"],
            Format::OpenExr => &["exr"],
            Format::Farbfeld => &["ff"],
            Format::Qoi => &["qoi"],
            Format::Pcx => &["pcx"],
        }
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Format::Svg => "svg",
            Format::Avif => "avif",
            Format::Bmp => "bmp",
            Format::Dds => "dds",
            Format::Farbfeld => "ff",
            Format::Gif => "gif",
            Format::Hdr => "hdr",
            Format::Ico => "ico",
            Format::Jpeg => "jpeg",
            Format::OpenExr => "exr",
            Format::Pcx => "pcx",
            Format::Pnm => "pnm",
            Format::Png => "png",
            Format::Qoi => "qoi",
            Format::Tga => "tga",
            Format::Tiff => "tiff",
            Format::Webp => "webp",
        };
        f.write_str(s)
    }
}

impl TryFrom<&str> for Format {
    type Error = ImageError;

    fn try_from(ext: &str) -> Result<Self, Self::Error> {
        let e = normalise_ext(ext);
        let fmt = match e.as_str() {
            "svg" => Format::Svg,
            "avif" => Format::Avif,
            "bmp" => Format::Bmp,
            "dds" => Format::Dds,
            "ff" => Format::Farbfeld,
            "gif" => Format::Gif,
            "hdr" | "pic" => Format::Hdr,
            "ico" | "cur" => Format::Ico,
            "jpg" | "jpeg" => Format::Jpeg,
            "exr" => Format::OpenExr,
            "pcx" => Format::Pcx,
            "pnm" | "pbm" | "pgm" | "ppm" => Format::Pnm,
            "png" => Format::Png,
            "qoi" => Format::Qoi,
            "tga" => Format::Tga,
            "tiff" | "tif" => Format::Tiff,
            "webp" => Format::Webp,

            _ => return Err(ValidationError::UnsupportedExtension(ext.to_string()).into()),
        };
        Ok(fmt)
    }
}

impl TryFrom<String> for Format {
    type Error = ImageError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Format::try_from(s.as_str())
    }
}

impl TryFrom<&Path> for Format {
    type Error = ImageError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        match path.extension() {
            Some(os) => {
                let ext_str = os
                    .to_str()
                    .ok_or_else(|| ValidationError::InvalidExtensionFormat(os.to_os_string()))?;
                Format::try_from(ext_str)
            }
            None => Err(ValidationError::MissingExtensionForPath(path.to_path_buf()).into()),
        }
    }
}

impl TryFrom<PathBuf> for Format {
    type Error = ImageError;
    fn try_from(p: PathBuf) -> Result<Self, Self::Error> {
        Format::try_from(p.as_path())
    }
}

impl TryFrom<Format> for image::ImageFormat {
    type Error = ImageError;

    fn try_from(f: Format) -> Result<Self, Self::Error> {
        use image::ImageFormat as IF;
        match f {
            Format::Png => Ok(IF::Png),
            Format::Jpeg => Ok(IF::Jpeg),
            Format::Gif => Ok(IF::Gif),
            Format::Webp => Ok(IF::WebP),
            Format::Tiff => Ok(IF::Tiff),
            Format::Avif => Ok(IF::Avif),
            Format::Pnm => Ok(IF::Pnm),
            Format::Tga => Ok(IF::Tga),
            Format::Dds => Ok(IF::Dds),
            Format::Bmp => Ok(IF::Bmp),
            Format::Ico => Ok(IF::Ico),
            Format::Hdr => Ok(IF::Hdr),
            Format::OpenExr => Ok(IF::OpenExr),
            Format::Farbfeld => Ok(IF::Farbfeld),
            Format::Qoi => Ok(IF::Qoi),
            Format::Pcx => Ok(IF::Pcx),
            other => Err(ErrorKind::Internal(format!(
                "format {:?} cannot be converted to image::ImageFormat",
                other
            ))
            .into()),
        }
    }
}
