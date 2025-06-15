use image::{guess_format, GenericImageView};

use crate::{ImageFormat, Img, ImgError, ImgSrc, Result, TransformPipeline};

impl Img {
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        let guessed_format = guess_format(&bytes).map_err(|_| ImgError::GuessFormat)?;
        let format = ImageFormat::try_from(guessed_format)?;

        let img = image::load_from_memory(&bytes).map_err(|e| ImgError::Decoding {
            id: "raw bytes".to_string(),
            source: e,
            format,
        })?;

        let (width, height) = img.dimensions();

        Ok(Self {
            img,
            src: ImgSrc::Bytes,
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
    use super::*;
    use std::fs;

    #[test]
    fn test_img_from_valid_bytes() {
        let bytes = fs::read("tests/assets/test.png").expect("test image should exist");
        let img = Img::from_bytes(bytes).expect("Image should be loaded from bytes");

        assert!(img.width > 0);
        assert!(img.height > 0);
        assert_eq!(img.aspect_ratio, img.width as f32 / img.height as f32);

        assert!(
            matches!(img.src, ImgSrc::Bytes { .. }),
            "Expected ImgSrc::Bytes"
        );

        assert_eq!(img.format, ImageFormat::Png, "Image format should be PNG");
    }

    #[test]
    fn test_img_from_valid_bytes_multiple_formats() {
        let test_cases = vec![
            ("test.png", ImageFormat::Png),
            ("test.jpg", ImageFormat::Jpeg),
            ("test.webp", ImageFormat::WebP),
        ];

        for (filename, expected_format) in test_cases {
            let path = format!("tests/assets/{}", filename);
            let bytes = fs::read(&path).expect(&format!("Image file {} should exist", filename));

            let img = Img::from_bytes(bytes).expect("Image should be loaded from bytes");

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
