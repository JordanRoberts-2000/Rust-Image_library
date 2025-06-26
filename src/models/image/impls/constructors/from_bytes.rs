use std::io::Cursor;

use image::ImageReader;

use crate::{Image, ImageConfig, ImageData, ImageError, ImageFormat, ImageSrc, Result};

impl Image {
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        let reader = ImageReader::new(Cursor::new(&bytes))
            .with_guessed_format()
            .map_err(|_| ImageError::FormatDetectionFailed)?;

        let format =
            ImageFormat::try_from(reader.format().ok_or_else(|| ImageError::UnknownFormat)?)?;

        let (width, height) = reader
            .into_dimensions()
            .map_err(ImageError::DimensionsFailed)?;

        Ok(Self {
            src: ImageSrc::Bytes,
            data: ImageData::Bytes(bytes),
            config: ImageConfig::default(),
            height,
            width,
            aspect_ratio: width as f32 / height as f32,
            format,
        })
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::fs;

//     #[test]
//     fn test_img_from_valid_bytes() {
//         let bytes = fs::read("tests/assets/test.png").expect("test image should exist");
//         let img = Image::from_bytes(bytes).expect("Image should be loaded from bytes");

//         assert!(img.width > 0);
//         assert!(img.height > 0);
//         assert_eq!(img.aspect_ratio, img.width as f32 / img.height as f32);

//         assert!(
//             matches!(img.src, ImageSrc::Bytes { .. }),
//             "Expected ImgSrc::Bytes"
//         );

//         assert_eq!(img.format, ImageFormat::Png, "Image format should be PNG");
//     }

//     #[test]
//     fn test_img_from_valid_bytes_multiple_formats() {
//         let test_cases = vec![
//             ("test.png", ImageFormat::Png),
//             ("test.jpg", ImageFormat::Jpeg),
//             ("test.webp", ImageFormat::WebP),
//         ];

//         for (filename, expected_format) in test_cases {
//             let path = format!("tests/assets/{}", filename);
//             let bytes = fs::read(&path).expect(&format!("Image file {} should exist", filename));

//             let img = Image::from_bytes(bytes).expect("Image should be loaded from bytes");

//             assert_eq!(
//                 img.format, expected_format,
//                 "Image format mismatch for {}",
//                 filename
//             );
//         }
//     }

//     #[test]
//     fn test_img_from_invalid_bytes_should_error() {
//         let fake_bytes = b"this is not an image".to_vec();

//         let result = Image::from_bytes(fake_bytes);

//         match result {
//             Err(ImageError::GuessFormat) => {}
//             Err(ImageError::Decoding { .. }) => {}
//             _ => panic!("Expected GuessFormat or Decoding error"),
//         }
//     }
// }
