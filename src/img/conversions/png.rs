use {
    image::{load_from_memory, ImageFormat},
    std::io::Cursor,
};

use crate::{Img, ImgError, Result};

impl Img {
    pub fn png(&mut self) -> Result<&mut Self> {
        if self.format == ImageFormat::Png {
            return Ok(self);
        }

        let mut buffer = Vec::new();
        self.img
            .write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)
            .map_err(|e| ImgError::Conversion {
                source: e,
                id: self.id(),
                format: ImageFormat::Png,
            })?;

        self.img = load_from_memory(&buffer).map_err(|e| ImgError::Decoding {
            id: self.id(),
            source: e,
            format: ImageFormat::Png,
        })?;

        self.format = ImageFormat::Png;
        self.update_extension("png");

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::ImageFormat;
    use std::fs;

    #[test]
    fn test_img_to_png_from_all_formats() {
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
                "Expected original format to match for {}",
                filename
            );

            let result = img.png();
            assert!(
                result.is_ok(),
                "PNG conversion should succeed for {}",
                filename
            );

            assert_eq!(
                img.format,
                ImageFormat::Png,
                "Format should be PNG after conversion for {}",
                filename
            );

            assert!(
                img.file_name().unwrap().ends_with(".png"),
                "File name should end with .png for {}. Got: {}",
                filename,
                img.file_name().unwrap()
            );
        }
    }
}
