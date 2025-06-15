use image::{GenericImageView, ImageReader};
use std::path::Path;

use crate::{
    utils::validation::ensure_existing_image_file, ImageFormat, Img, ImgError, ImgSrc, Result,
    TransformPipeline,
};

impl Img {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        ensure_existing_image_file(&path)?;

        let reader = ImageReader::open(path).map_err(|e| ImgError::Open {
            source: e,
            path: path.to_path_buf(),
        })?;

        let format = ImageFormat::try_from(reader.format().ok_or_else(|| ImgError::GuessFormat)?)?;

        let img = reader.decode().map_err(|e| ImgError::DecodeFile {
            source: e,
            path: path.to_path_buf(),
        })?;

        let (width, height) = img.dimensions();

        Ok(Self {
            img,
            src: ImgSrc::File {
                path: path.to_path_buf(),
            },
            transform_pipeline: TransformPipeline::default(),
            height,
            width,
            aspect_ratio: width as f32 / height as f32,
            format,
        })
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use crate::ValidationError;

    use super::*;
    use std::{fs::File, path::PathBuf};

    #[test]
    fn test_img_from_file_success() {
        let path = PathBuf::from("tests/assets/test.png");
        let img = Img::from_file(&path).expect("Image should open successfully");

        assert_eq!(img.format, ImageFormat::Png);

        let expected_ratio = img.width as f32 / img.height as f32;
        assert!((img.aspect_ratio - expected_ratio).abs() < 0.01);
    }

    #[test]
    fn test_img_from_file_multiple_formats() {
        let cases = [
            ("test.png", ImageFormat::Png),
            ("test.jpg", ImageFormat::Jpeg),
            ("test.webp", ImageFormat::WebP),
        ];

        for (file, fmt) in cases {
            let path = PathBuf::from(format!("tests/assets/{}", file));
            let img = Img::from_file(&path).expect(&format!("Should open {}", file));
            assert_eq!(img.format, fmt);
        }
    }

    #[test]
    fn test_img_from_file_nonexistent_path() {
        let path = PathBuf::from("tests/assets/does_not_exist.png");
        let err = Img::from_file(&path).unwrap_err();
        match err {
            ImgError::Validation(ValidationError::PathNotFound(p)) => assert_eq!(p, path),
            _ => panic!("Expected ImgError::Validation(PathNotFound), got {:?}", err),
        }
    }

    #[test]
    fn test_img_from_file_directory() {
        let dir = Path::new("tests/assets");
        let err = Img::from_file(dir).unwrap_err();
        match err {
            ImgError::Validation(ValidationError::NotAFile(p)) => assert_eq!(p, dir.to_path_buf()),
            _ => panic!("Expected ImgError::Validation(NotAFile), got {:?}", err),
        }
    }

    #[test]
    fn test_img_from_file_missing_extension() {
        let tmp = TempDir::new().unwrap();
        let file = tmp.path().join("no_ext");
        File::create(&file).unwrap();

        let err = Img::from_file(&file).unwrap_err();
        match err {
            ImgError::Validation(ValidationError::MissingExtension(p)) => assert_eq!(p, file),
            _ => panic!(
                "Expected ImgError::Validation(MissingExtension), got {:?}",
                err
            ),
        }
    }

    #[test]
    fn test_img_from_file_unsupported_extension() {
        let tmp = TempDir::new().unwrap();
        let file = tmp.path().join("foo.txt");
        File::create(&file).unwrap();

        let err = Img::from_file(&file).unwrap_err();
        match err {
            ImgError::Validation(ValidationError::NotAnImageFile(p)) => assert_eq!(p, file),
            _ => panic!(
                "Expected ImgError::Validation(NotAnImageFile), got {:?}",
                err
            ),
        }
    }
}
