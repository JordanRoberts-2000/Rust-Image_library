use {
    crate::{
        constants::MAGIC_BYTES_READ_LIMIT,
        format_detection::{Guessable, Guesser, Signature},
        utils::normalise_ext,
        Format, FormatOps, ImageError, ImageFormat, ValidationError,
    },
    inherent::inherent,
    std::{
        fmt,
        io::BufRead,
        path::{Path, PathBuf},
        str,
    },
};

impl ImageFormat {
    pub(crate) fn signatures(&self) -> &'static [Signature] {
        match self {
            ImageFormat::Png => {
                &[Signature { pattern: b"\x89PNG\r\n\x1A\n", mask: None, offset: 0 }]
            }
            ImageFormat::Jpeg => {
                &[Signature { pattern: &[0xFF, 0xD8, 0xFF], mask: None, offset: 0 }]
            }
            ImageFormat::Gif => &[
                Signature { pattern: b"GIF89a", mask: None, offset: 0 },
                Signature { pattern: b"GIF87a", mask: None, offset: 0 },
            ],
            ImageFormat::Webp => &[Signature {
                pattern: b"RIFF\x00\x00\x00\x00WEBP",
                mask: Some(b"\xFF\xFF\xFF\xFF\x00\x00\x00\x00\xFF\xFF\xFF\xFF"),
                offset: 0,
            }],
            ImageFormat::Tiff => &[
                Signature { pattern: b"II*\x00", mask: None, offset: 0 },
                Signature { pattern: b"MM\x00*", mask: None, offset: 0 },
            ],
            ImageFormat::Avif => &[Signature {
                pattern: b"\x00\x00\x00\x00ftypavif",
                mask: Some(b"\x00\x00\x00\x00\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF"),
                offset: 0,
            }],
            ImageFormat::Pnm => &[
                Signature { pattern: b"P1", mask: None, offset: 0 },
                Signature { pattern: b"P2", mask: None, offset: 0 },
                Signature { pattern: b"P3", mask: None, offset: 0 },
                Signature { pattern: b"P4", mask: None, offset: 0 },
                Signature { pattern: b"P5", mask: None, offset: 0 },
                Signature { pattern: b"P6", mask: None, offset: 0 },
                Signature { pattern: b"P7", mask: None, offset: 0 },
            ],
            ImageFormat::Tga => &[],
            ImageFormat::Dds => &[Signature { pattern: b"DDS ", mask: None, offset: 0 }],
            ImageFormat::Bmp => &[Signature { pattern: b"BM", mask: None, offset: 0 }],
            ImageFormat::Ico => &[Signature { pattern: &[0, 0, 1, 0], mask: None, offset: 0 }],
            ImageFormat::Hdr => &[Signature { pattern: b"#?RADIANCE", mask: None, offset: 0 }],
            ImageFormat::OpenExr => {
                &[Signature { pattern: &[0x76, 0x2F, 0x31, 0x01], mask: None, offset: 0 }]
            }
            ImageFormat::Farbfeld => &[Signature { pattern: b"farbfeld", mask: None, offset: 0 }],
            ImageFormat::Qoi => &[Signature { pattern: b"qoif", mask: None, offset: 0 }],
            ImageFormat::Pcx => {
                &[Signature { pattern: &[0x0A, 0x00], mask: Some(&[0xFF, 0xF8]), offset: 0 }]
            }
        }
    }
}

impl Guessable for ImageFormat {
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

impl ImageFormat {
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
            "webp" => ImageFormat::Webp,
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
            I::WebP => Ok(ImageFormat::Webp),
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
            ImageFormat::Webp => I::WebP,
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
