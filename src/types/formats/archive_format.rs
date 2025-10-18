use {
    crate::{
        constants::MAGIC_BYTES_READ_LIMIT,
        format_detection::{Guessable, Guesser, Signature},
        utils::normalise_ext,
        FormatOps, ImageError,
    },
    inherent::inherent,
    std::{fmt, io::BufRead, path::Path},
    strum_macros::EnumIter,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, EnumIter)]
pub enum ArchiveFormat {
    Zip,
    Tar,
    TarGz,
}

impl ArchiveFormat {
    pub(crate) fn signatures(&self) -> &'static [Signature] {
        match self {
            ArchiveFormat::Zip => &[
                Signature { pattern: b"PK\x03\x04", mask: None, offset: 0 },
                Signature { pattern: b"PK\x05\x06", mask: None, offset: 0 },
                Signature { pattern: b"PK\x07\x08", mask: None, offset: 0 },
            ],
            ArchiveFormat::Tar => &[
                Signature { pattern: b"ustar\x00", mask: None, offset: 257 },
                Signature { pattern: b"ustar ", mask: None, offset: 257 },
            ],
            // GZIP: 0x1F 0x8B at start (used for .tar.gz / .tgz)
            // Note: this detects gzip in general; to *confirm* tar.gz you’d need to
            // decompress a little and check for the TAR magic at offset 257 in the payload.
            ArchiveFormat::TarGz => &[Signature { pattern: &[0x1F, 0x8B], mask: None, offset: 0 }],
        }
    }
}

impl Guessable for ArchiveFormat {
    fn guess(&self, bytes: &[u8]) -> bool {
        self.signatures().iter().any(|s| s.matches(bytes))
    }

    fn ext_guess(&self, ext: &str) -> bool {
        self.extensions().contains(&normalise_ext(ext).as_str())
    }

    fn read_limit(&self) -> usize {
        MAGIC_BYTES_READ_LIMIT
    }
}

impl ArchiveFormat {
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
impl FormatOps for ArchiveFormat {
    pub fn all() -> Vec<Self> {
        vec![ArchiveFormat::Zip, ArchiveFormat::Tar, ArchiveFormat::TarGz]
    }

    pub fn supported_exts() -> Vec<&'static str> {
        vec!["zip", "tar", "gz", "tgz", "tar.gz"]
    }

    pub fn is_supported_ext(ext: &str) -> bool {
        matches!(ext.to_lowercase().as_str(), "zip" | "tar" | "gz" | "tgz" | "tar.gz")
    }

    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "zip" => Some(ArchiveFormat::Zip),
            "tar" => Some(ArchiveFormat::Tar),
            "gz" | "tgz" | "tar.gz" => Some(ArchiveFormat::TarGz),
            _ => None,
        }
    }

    pub fn from_mime(mime: &str) -> Option<Self> {
        match mime {
            "application/zip" => Some(ArchiveFormat::Zip),
            "application/x-tar" => Some(ArchiveFormat::Tar),
            "application/gzip" | "application/x-gzip" => Some(ArchiveFormat::TarGz),
            _ => None,
        }
    }

    pub fn mime_type(&self) -> &'static str {
        match self {
            ArchiveFormat::Zip => "application/zip",
            ArchiveFormat::Tar => "application/x-tar",
            ArchiveFormat::TarGz => "application/gzip",
        }
    }

    pub fn primary_extension(&self) -> &'static str {
        match self {
            ArchiveFormat::Zip => "zip",
            ArchiveFormat::Tar => "tar",
            ArchiveFormat::TarGz => "tar.gz",
        }
    }

    pub fn extensions(&self) -> &'static [&'static str] {
        match self {
            ArchiveFormat::Zip => &["zip"],
            ArchiveFormat::Tar => &["tar"],
            ArchiveFormat::TarGz => &["tar.gz", "tgz", "gz"],
        }
    }
}

impl fmt::Display for ArchiveFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ArchiveFormat::Zip => "zip",
            ArchiveFormat::Tar => "tar",
            ArchiveFormat::TarGz => "tar.gz",
        };
        f.write_str(s)
    }
}
