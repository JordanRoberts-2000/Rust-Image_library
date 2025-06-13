use std::path::PathBuf;

use image::{guess_format, GenericImageView};
use uuid::Uuid;

use crate::{enums::ImgSrc, Img, ImgError, Result};

impl Img {
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        let id = Uuid::new_v4().to_string();
        let format = guess_format(&bytes).map_err(|_| ImgError::GuessFormat)?;
        let size_bytes = bytes.len();

        let img = image::load_from_memory(&bytes).map_err(|e| ImgError::Decoding {
            id: id.clone(),
            source: e,
            format,
        })?;

        let (width, height) = img.dimensions();

        let ext = match format.extensions_str().first() {
            Some(ext) => ext,
            None => return Err(ImgError::ExtensionInvalid),
        };
        let file_name = format!("{}.{}", id, ext);

        let cwd = PathBuf::from(".");
        let target = cwd.join(&file_name);

        Ok(Self {
            img,
            src: ImgSrc::Bytes { id: id.clone() },
            target_path: target,
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
    use super::*;
    use std::fs;

    #[test]
    fn test_img_from_valid_bytes() {
        let bytes = fs::read("tests/assets/test.png").expect("test image should exist");
        let img = Img::from_bytes(bytes).expect("Image should be loaded from bytes");

        assert!(img.width > 0);
        assert!(img.height > 0);
        assert!(img.size_bytes > 0);
        assert_eq!(img.aspect_ratio, img.width as f32 / img.height as f32);

        assert!(
            img.file_name().unwrap().ends_with(".png"),
            "Expected file_name to end with .png, got: {}",
            img.file_name().unwrap()
        );

        assert!(
            matches!(img.src, ImgSrc::Bytes { .. }),
            "Expected ImgSrc::Bytes"
        );

        assert_eq!(
            img.format,
            image::ImageFormat::Png,
            "Image format should be PNG"
        );
    }

    #[test]
    fn test_img_from_valid_bytes_multiple_formats() {
        use image::ImageFormat;

        let test_cases = vec![
            ("test.png", ImageFormat::Png),
            ("test.jpg", ImageFormat::Jpeg),
            ("test.webp", ImageFormat::WebP),
        ];

        for (filename, expected_format) in test_cases {
            let path = format!("tests/assets/{}", filename);
            let bytes = fs::read(&path).expect(&format!("Image file {} should exist", filename));

            let img = Img::from_bytes(bytes).expect("Image should be loaded from bytes");

            assert!(
                img.file_name()
                    .unwrap()
                    .ends_with(expected_format.extensions_str()[0]),
                "File name should end with .{} for {}, got: {}",
                expected_format.extensions_str()[0],
                filename,
                img.file_name().unwrap()
            );

            assert_eq!(
                img.format, expected_format,
                "Image format mismatch for {}",
                filename
            );
        }
    }

    #[test]
    fn test_img_from_invalid_bytes_should_error() {
        let fake_bytes = b"this is not an image".to_vec();

        let result = Img::from_bytes(fake_bytes);

        match result {
            Err(ImgError::GuessFormat) => {}
            Err(ImgError::Decoding { .. }) => {}
            _ => panic!("Expected GuessFormat or Decoding error"),
        }
    }
}
