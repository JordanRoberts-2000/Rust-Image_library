use crate::{Img, ImgError, Result};

use image::{codecs::jpeg::JpegEncoder, load_from_memory, ExtendedColorType, ImageFormat};

impl Img {
    pub fn jpeg(&mut self, quality: u8) -> Result<&mut Self> {
        if self.format == ImageFormat::Jpeg && quality == 100 {
            return Ok(self);
        }

        let mut buffer = Vec::new();
        let mut encoder = JpegEncoder::new_with_quality(&mut buffer, quality);

        encoder
            .encode(
                self.img.to_rgb8().as_raw(),
                self.width,
                self.height,
                ExtendedColorType::Rgb8,
            )
            .map_err(|e| ImgError::Conversion {
                source: e,
                id: self.id(),
                format: ImageFormat::Jpeg,
            })?;

        // Note: This decoding will re-read the JPEG data, so any JPEG-specific encoding
        // (like progressive settings) will be lost.
        self.img = load_from_memory(&buffer).map_err(|e| ImgError::Decoding {
            id: self.id(),
            source: e,
            format: ImageFormat::Jpeg,
        })?;

        self.format = ImageFormat::Jpeg;
        self.update_extension("jpg");

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::ImageFormat;
    use std::fs;

    #[test]
    fn test_img_to_jpeg_from_all_formats() {
        let test_cases = vec![
            ("test.png", ImageFormat::Png),
            ("test.webp", ImageFormat::WebP),
            ("test.jpg", ImageFormat::Jpeg),
        ];

        for (filename, original_format) in test_cases {
            let path = format!("tests/assets/{}", filename);
            let bytes = fs::read(&path).expect(&format!("Image file {} should exist", filename));

            let mut img = Img::from_bytes(bytes).expect(&format!("Should load {}", filename));

            assert_eq!(
                img.format, original_format,
                "Expected format to match original for {}",
                filename
            );

            let result = img.jpeg(80);
            assert!(
                result.is_ok(),
                "JPEG conversion should succeed for {}",
                filename
            );

            assert_eq!(
                img.format,
                ImageFormat::Jpeg,
                "Format should be JPEG after conversion for {}",
                filename
            );

            assert!(
                img.file_name().unwrap().ends_with(".jpg"),
                "File name should end with .jpg for {}. Got: {}",
                filename,
                img.file_name().unwrap()
            );
        }
    }
}
