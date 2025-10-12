use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    iter,
    path::Path,
    str,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Format {
    Svg,
    Png,
    Jpeg,
    Gif,
    WebP,
    Tiff,
    Avif,
    Pnm,
    Tga,
    Dds,
    Bmp,
    Ico,
    Hdr,
    OpenExr,
    Farbfeld,
    Qoi,
    Pcx,
}

const MAGIC_BYTES_BUFFER_SIZE: usize = 16;
const SVG_DETECTION_BUFFER_SIZE: usize = 512;

static MAGIC_BYTES: [(&[u8], &[u8], Format); 23] = [
    (b"\x89PNG\r\n\x1a\n", b"", Format::Png),
    (&[0xff, 0xd8, 0xff], b"", Format::Jpeg),
    (b"GIF89a", b"", Format::Gif),
    (b"GIF87a", b"", Format::Gif),
    (b"RIFF\0\0\0\0WEBP", b"\xFF\xFF\xFF\xFF\0\0\0\0", Format::WebP),
    (b"MM\x00*", b"", Format::Tiff),
    (b"II*\x00", b"", Format::Tiff),
    (b"DDS ", b"", Format::Dds),
    (b"BM", b"", Format::Bmp),
    (&[0, 0, 1, 0], b"", Format::Ico),
    (b"#?RADIANCE", b"", Format::Hdr),
    (b"\0\0\0\0ftypavif", b"\xFF\xFF\0\0", Format::Avif),
    (&[0x76, 0x2f, 0x31, 0x01], b"", Format::OpenExr),
    (b"qoif", b"", Format::Qoi),
    (b"P1", b"", Format::Pnm),
    (b"P2", b"", Format::Pnm),
    (b"P3", b"", Format::Pnm),
    (b"P4", b"", Format::Pnm),
    (b"P5", b"", Format::Pnm),
    (b"P6", b"", Format::Pnm),
    (b"P7", b"", Format::Pnm),
    (b"farbfeld", b"", Format::Farbfeld),
    (&[0x0a, 0x0], b"\xFF\xF8", Format::Pcx),
];

impl Format {
    pub fn guess_from_file(path: impl AsRef<Path>) -> io::Result<Option<Self>> {
        let mut r = BufReader::new(File::open(path)?);
        Self::guess_from_reader(&mut r)
    }

    pub fn guess_from_reader<R: BufRead>(r: &mut R) -> io::Result<Option<Self>> {
        let buf = r.fill_buf()?;
        Self::guess_from_bytes(buf)
    }

    pub fn guess_from_bytes(bytes: impl AsRef<[u8]>) -> io::Result<Option<Self>> {
        let bytes = bytes.as_ref();

        let magic_bytes = bytes
            .get(..MAGIC_BYTES_BUFFER_SIZE)
            .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "insufficient bytes"))?;

        if let Some(format) = Self::detect_binary_format(&magic_bytes) {
            return Ok(Some(format));
        }

        let svg_detection_bytes = &bytes[..bytes.len().min(SVG_DETECTION_BUFFER_SIZE)];
        if Self::can_detect_svg(&svg_detection_bytes) {
            return Ok(Some(Self::Svg));
        }

        Ok(None)
    }

    fn can_detect_svg(bytes: &[u8]) -> bool {
        let text = match str::from_utf8(bytes) {
            Ok(t) => t,
            Err(_) => return false,
        };

        let trimmed = text.trim_start();

        if trimmed.starts_with("<svg") {
            return true;
        }

        if trimmed.starts_with("<?xml") {
            return text.contains("<svg");
        }

        false
    }

    fn detect_binary_format(bytes: &[u8]) -> Option<Self> {
        for &(signature, mask, format) in &MAGIC_BYTES {
            if mask.is_empty() {
                if bytes.starts_with(signature) {
                    return Some(format);
                }
            } else if bytes.len() >= signature.len()
                && bytes
                    .iter()
                    .zip(signature.iter())
                    .zip(mask.iter().chain(iter::repeat(&0xFF)))
                    .all(|((&byte, &sig), &mask)| byte & mask == sig)
            {
                return Some(format);
            }
        }

        None
    }
}
