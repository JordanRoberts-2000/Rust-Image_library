use {
    crate::{ErrorKind, ImageFormat, Result},
    image::{DynamicImage, ImageReader},
    std::path::Path,
};

pub fn from_path(path: impl AsRef<Path>) -> Result<(DynamicImage, ImageFormat)> {
    let p = path.as_ref();

    let reader = ImageReader::open(p)
        .map_err(|e| ErrorKind::Open { source: e, path: p.to_path_buf() })?
        .with_guessed_format()
        .map_err(ErrorKind::FormatDetectionFailed)?;

    let ext_fmt = reader.format().ok_or(ErrorKind::UnknownFormat)?;
    let format = ImageFormat::try_from(ext_fmt)?;

    let img =
        reader.decode().map_err(|e| ErrorKind::DecodeFile { source: e, path: p.to_path_buf() })?;

    Ok((img, format))
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            test_utils::{
                corrupted_header_image_file, corrupted_image_file, image_file,
                MOCK_IMAGE_DIMENSIONS,
            },
            ErrorKind, ImageFormat,
        },
        image::GenericImageView,
        std::fs,
        strum::IntoEnumIterator,
        tempfile::TempDir,
    };

    #[test]
    fn from_path_all_formats_ok() {
        let tmp = TempDir::new().expect("make temp dir");

        for fmt in ImageFormat::iter() {
            let p = image_file(&tmp, fmt);

            let (img, returned_fmt) = from_path(&p)
                .unwrap_or_else(|e| panic!("from_path failed for {fmt:?} at {}: {e}", p.display()));

            assert_eq!(img.dimensions(), MOCK_IMAGE_DIMENSIONS, "wrong dimensions for {fmt:?}");
            assert_eq!(fmt, returned_fmt, "wrong format for {fmt:?}");
        }
    }

    #[test]
    fn from_path_nonexistent_err() {
        let tmp = TempDir::new().expect("make temp dir");
        let p = tmp.path().join("does_not_exist.png");

        let err = from_path(&p).expect_err("nonexistent path should error");

        match err.kind() {
            ErrorKind::Open { path, .. } => {
                assert_eq!(path, &p);
            }
            other => panic!("expected ErrorKind::Open, got {:?}", other),
        }
    }

    #[test]
    fn from_path_empty_file_format_guess_err() {
        let tmp = TempDir::new().expect("make temp dir");
        let p = tmp.path().join("empty.img");
        // create zero-length file
        fs::File::create(&p).expect("create empty file");

        let err = from_path(&p).expect_err("empty file should fail format guess");

        assert!(
            matches!(err.kind(), ErrorKind::FormatDetectionFailed(_)),
            "expected FormatDetectionFailed(_), got {:?}",
            err.kind()
        );
    }

    #[test]
    fn from_path_corrupted_decode_err() {
        let tmp = TempDir::new().expect("make temp dir");
        let p = corrupted_image_file(&tmp, ImageFormat::Png);

        let err = from_path(&p).expect_err("corrupted PNG should fail to decode");

        match err.kind() {
            ErrorKind::DecodeFile { path, .. } => {
                assert_eq!(path, &p);
            }
            other => panic!("expected DecodeFile {{ .. }}, got {:?}", other),
        }
    }

    #[test]
    fn from_path_corrupted_headers_decode_err() {
        let tmp = TempDir::new().expect("make temp dir");
        let p = corrupted_header_image_file(&tmp, ImageFormat::Png);

        let err = from_path(&p).expect_err("corrupted PNG should fail to decode");

        match err.kind() {
            ErrorKind::DecodeFile { path, .. } => {
                assert_eq!(path, &p);
            }
            other => panic!("expected DecodeFile {{ .. }}, got {:?}", other),
        }
    }
}
