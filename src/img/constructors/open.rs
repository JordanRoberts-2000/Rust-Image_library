use image::{guess_format, GenericImageView};
use std::{fs, path::Path};

use crate::{enums::ImgSrc, Img, ImgError, IoError, Result};

impl Img {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        let img = image::open(path).map_err(|e| ImgError::Open {
            source: e,
            path: path.to_path_buf(),
        })?;

        let (width, height) = img.dimensions();

        let bytes = fs::read(path).map_err(|e| IoError::ReadFile(e, path.to_path_buf()))?;

        let size_bytes = bytes.len();
        let format = guess_format(&bytes).map_err(|_| ImgError::GuessFormat)?;

        Ok(Self {
            img,
            src: ImgSrc::Local {
                path: path.to_path_buf(),
            },
            target_path: path.to_path_buf(),
            height,
            width,
            aspect_ratio: width as f32 / height as f32,
            format,
            size_bytes,
        })
    }
}

#[cfg(test)]
mod tests {
    use image::ImageFormat;

    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_img_open() {
        let path = PathBuf::from("tests/assets/test.png");
        let img = Img::open(&path).expect("Image should open successfully");

        assert_eq!(img.target_path, path);
        match &img.src {
            ImgSrc::Local { path: src_path } => assert_eq!(src_path, &img.target_path),
            _ => panic!("Expected ImgSrc::Local"),
        }

        assert_eq!(
            img.file_name().unwrap(),
            "test.png",
            "Filename should match"
        );

        assert_eq!(img.format, ImageFormat::Png, "Image format should be PNG");

        assert!(img.width > 0, "Width should be greater than 0");
        assert!(img.height > 0, "Height should be greater than 0");
        assert!(img.size_bytes > 0, "Size in bytes should be greater than 0");

        let expected_ratio = img.width as f32 / img.height as f32;
        let delta = (img.aspect_ratio - expected_ratio).abs();
        assert!(delta < 0.01, "Aspect ratio delta too large: {}", delta);
    }

    #[test]
    fn test_img_open_multiple_formats() {
        use image::ImageFormat;
        use std::path::PathBuf;

        let test_cases = vec![
            ("test.png", ImageFormat::Png),
            ("test.jpg", ImageFormat::Jpeg),
            ("test.webp", ImageFormat::WebP),
        ];

        for (filename, expected_format) in test_cases {
            let path = PathBuf::from(format!("tests/assets/{}", filename));
            let img = Img::open(&path).expect(&format!("Should open {}", filename));

            assert_eq!(
                img.file_name().unwrap(),
                filename,
                "Filename should match for {}",
                filename
            );

            assert_eq!(
                img.format, expected_format,
                "Image format should be correct for {}",
                filename
            );
        }
    }

    #[test]
    fn test_img_open_nonexistent_file_should_error() {
        let path = PathBuf::from("tests/assets/does_not_exist.png");
        let result = Img::open(&path);

        match result {
            Err(ImgError::Open { path: err_path, .. }) => {
                assert_eq!(err_path, path, "Error should contain the failing path");
            }
            _ => panic!("Expected ImgError::Open for nonexistent file"),
        }
    }

    #[test]
    fn test_img_open_directory_should_error() {
        let path = Path::new("tests/assets");
        let result = Img::open(path);

        match result {
            Err(ImgError::Open { path: err_path, .. }) => {
                assert_eq!(err_path, path.to_path_buf());
            }
            Err(e) => panic!("Expected ImgError::Open for directory input, got error: {e:#?}"),
            Ok(..) => panic!("Expected ImgError::Open for directory input, instead got Ok"),
        }
    }

    #[test]
    fn test_img_open_with_non_image_file_should_error() {
        let path = Path::new("tests/assets/not_an_image.txt");
        std::fs::write(path, b"this is not an image").unwrap();

        let result = Img::open(path);
        assert!(matches!(result, Err(ImgError::Open { .. })));

        // Clean up
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_img_open_accepts_various_path_types() {
        let path_buf = PathBuf::from("tests/assets/test.png");
        let path = Path::new("tests/assets/test.png");
        let path_string = String::from("tests/assets/test.png");

        // Should all succeed without panicking
        Img::open(&path_buf).expect("PathBuf ref");
        Img::open(path_buf.clone()).expect("PathBuf value");
        Img::open(path).expect("Path ref");
        Img::open(path.to_path_buf()).expect("Path value");
        Img::open(&path_string).expect("String ref");
        Img::open(path_string.clone()).expect("String value");
        Img::open("tests/assets/test.png").expect("String literal");
    }
}
