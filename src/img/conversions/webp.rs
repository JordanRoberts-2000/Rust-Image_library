use {
    image::ImageFormat,
    webp::{Encoder, PixelLayout},
};

use crate::{Img, ImgError, Result};

impl Img {
    pub fn webp(&mut self) -> Result<&mut Self> {
        if self.format == ImageFormat::WebP {
            return Ok(self);
        }

        let rgba_image = self.img.to_rgba8();
        let encoder = Encoder::new(&rgba_image, PixelLayout::Rgba, self.width, self.height);

        let webp_data = encoder.encode_lossless();

        self.img =
            image::load_from_memory_with_format(&webp_data, ImageFormat::WebP).map_err(|e| {
                ImgError::Decoding {
                    id: self.id(),
                    source: e,
                    format: ImageFormat::WebP,
                }
            })?;

        self.format = ImageFormat::WebP;
        self.update_extension("webp");

        Ok(self)
    }
    pub fn webp_lossy(&mut self, quality: u8) -> Result<&mut Self> {
        if self.format == ImageFormat::WebP && quality == 100 {
            return Ok(self);
        }

        let rgba_image = self.img.to_rgba8();
        let encoder = Encoder::new(&rgba_image, PixelLayout::Rgba, self.width, self.height);

        let webp_data =
            encoder
                .encode_simple(false, quality as f32)
                .map_err(|e| ImgError::WebPConversion {
                    err: e,
                    id: self.id(),
                })?;

        self.img =
            image::load_from_memory_with_format(&webp_data, ImageFormat::WebP).map_err(|e| {
                ImgError::Decoding {
                    id: self.id(),
                    source: e,
                    format: ImageFormat::WebP,
                }
            })?;

        self.format = ImageFormat::WebP;
        self.update_extension("webp");

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::ImageFormat;
    use std::fs;

    #[test]
    fn test_img_to_webp_from_all_formats_lossless() {
        let test_cases = vec![
            ("test.png", ImageFormat::Png),
            ("test.jpg", ImageFormat::Jpeg),
            ("test.webp", ImageFormat::WebP),
        ];

        for (filename, original_format) in test_cases {
            let path = format!("tests/assets/{}", filename);
            let bytes = fs::read(&path).expect(&format!("Image file {} should exist", filename));

            let mut img = Img::from_bytes(bytes).expect(&format!("Should load {}", filename));

            assert_eq!(
                img.format, original_format,
                "Original format mismatch for {}",
                filename
            );

            img.webp()
                .expect(&format!("Lossless WebP conversion failed for {}", filename));

            assert_eq!(
                img.format,
                ImageFormat::WebP,
                "Format should be WebP after conversion for {}",
                filename
            );

            // Extension may not be updated in .webp() version
            assert!(
                img.file_name().unwrap().ends_with(".webp") || original_format == ImageFormat::WebP,
                "Expected file_name to end with .webp for {}. Got: {}",
                filename,
                img.file_name().unwrap()
            );
        }
    }

    #[test]
    fn test_img_to_webp_lossy_from_all_formats() {
        let test_cases = vec![
            ("test.png", ImageFormat::Png),
            ("test.jpg", ImageFormat::Jpeg),
            ("test.webp", ImageFormat::WebP),
        ];

        for (filename, original_format) in test_cases {
            let path = format!("tests/assets/{}", filename);
            let bytes = fs::read(&path).expect(&format!("Image file {} should exist", filename));

            let mut img = Img::from_bytes(bytes).expect(&format!("Should load {}", filename));

            assert_eq!(img.format, original_format);

            img.webp_lossy(75)
                .expect(&format!("Lossy WebP conversion failed for {}", filename));

            assert_eq!(img.format, ImageFormat::WebP);

            assert!(
                img.file_name().unwrap().ends_with(".webp"),
                "File name should end with .webp for {}. Got: {}",
                filename,
                img.file_name().unwrap()
            );
        }
    }
}
