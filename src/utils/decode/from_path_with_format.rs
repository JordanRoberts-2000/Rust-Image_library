use {
    crate::{ErrorKind, ImageFormat, Result},
    image::{DynamicImage, ImageReader},
    std::path::Path,
};

pub fn from_path_with_format(path: impl AsRef<Path>, format: ImageFormat) -> Result<DynamicImage> {
    let p = path.as_ref();
    let mut reader =
        ImageReader::open(p).map_err(|e| ErrorKind::Open { source: e, path: p.to_path_buf() })?;

    reader.set_format(format.into());

    reader.decode().map_err(|e| ErrorKind::DecodeFile { source: e, path: p.to_path_buf() }.into())
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
    fn from_path_with_format_all_formats_ok() {
        let tmp = TempDir::new().expect("make temp dir");

        for fmt in ImageFormat::iter() {
            let p = image_file(&tmp, fmt);

            let img = from_path_with_format(&p, fmt).unwrap_or_else(|e| {
                panic!("from_path_with_format failed for {fmt:?} at {}: {e}", p.display())
            });

            assert_eq!(img.dimensions(), MOCK_IMAGE_DIMENSIONS, "wrong dimensions for {fmt:?}");
        }
    }

    #[test]
    fn from_path_with_format_nonexistent_err() {
        let tmp = TempDir::new().expect("make temp dir");
        let p = tmp.path().join("does_not_exist.png");

        let err =
            from_path_with_format(&p, ImageFormat::Png).expect_err("nonexistent path should error");

        match err.kind() {
            ErrorKind::Open { path, .. } => assert_eq!(path, &p),
            other => panic!("expected ErrorKind::Open, got {:?}", other),
        }
    }

    #[test]
    fn from_path_with_format_empty_file_decode_err() {
        let tmp = TempDir::new().expect("make temp dir");
        let p = tmp.path().join("empty.png");
        fs::File::create(&p).expect("create empty file");

        let err = from_path_with_format(&p, ImageFormat::Png)
            .expect_err("empty file should fail to decode");

        match err.kind() {
            ErrorKind::DecodeFile { path, .. } => assert_eq!(path, &p),
            other => panic!("expected DecodeFile {{ .. }}, got {:?}", other),
        }
    }

    #[test]
    fn from_path_with_format_corrupted_decode_err() {
        let tmp = TempDir::new().expect("make temp dir");
        let p = corrupted_image_file(&tmp, ImageFormat::Png);

        let err = from_path_with_format(&p, ImageFormat::Png)
            .expect_err("corrupted PNG should fail to decode");

        match err.kind() {
            ErrorKind::DecodeFile { path, .. } => assert_eq!(path, &p),
            other => panic!("expected DecodeFile {{ .. }}, got {:?}", other),
        }
    }

    #[test]
    fn from_path_with_format_corrupted_headers_decode_err() {
        let tmp = TempDir::new().expect("make temp dir");
        let p = corrupted_header_image_file(&tmp, ImageFormat::Png);

        let err = from_path_with_format(&p, ImageFormat::Png)
            .expect_err("header-corrupted PNG should fail to decode");

        match err.kind() {
            ErrorKind::DecodeFile { path, .. } => assert_eq!(path, &p),
            other => panic!("expected DecodeFile {{ .. }}, got {:?}", other),
        }
    }

    #[test]
    fn from_path_with_format_mismatch_decode_err() {
        let tmp = TempDir::new().expect("make temp dir");
        let p = image_file(&tmp, ImageFormat::Png);

        let err = from_path_with_format(&p, ImageFormat::Jpeg)
            .expect_err("PNG bytes forced as JPEG should fail to decode");

        match err.kind() {
            ErrorKind::DecodeFile { path, .. } => assert_eq!(path, &p),
            other => panic!("expected DecodeFile {{ .. }}, got {:?}", other),
        }
    }
}
