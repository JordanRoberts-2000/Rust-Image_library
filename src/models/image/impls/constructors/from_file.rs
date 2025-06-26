use std::path::PathBuf;

use {image::ImageReader, std::path::Path};

use crate::{
    constants::DEFAULT_IMAGE_FILE_NAME, utils::validation::ensure_existing_image_file, Image,
    ImageConfig, ImageData, ImageError, ImageFormat, ImageSrc, Result,
};

impl Image {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        ensure_existing_image_file(&path)?;

        let reader = ImageReader::open(path).map_err(|e| ImageError::Open {
            source: e,
            path: path.to_path_buf(),
        })?;

        let format =
            ImageFormat::try_from(reader.format().ok_or_else(|| ImageError::UnknownFormat)?)?;

        let (width, height) = reader
            .into_dimensions()
            .map_err(ImageError::DimensionsFailed)?;

        let file_name = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or(DEFAULT_IMAGE_FILE_NAME)
            .to_string();

        let parent_dir = path
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."));

        Ok(Self {
            src: ImageSrc::File(path.to_path_buf()),
            data: ImageData::File(path.to_path_buf()),
            config: ImageConfig {
                file_name,
                output_dir: parent_dir,
                ..Default::default()
            },
            height,
            width,
            aspect_ratio: width as f32 / height as f32,
            format,
        })
    }
}

// #[cfg(test)]
// mod tests {
//     use tempfile::TempDir;

//     use crate::ValidationError;

//     use super::*;
//     use std::{fs::File, path::PathBuf};

//     #[test]
//     fn test_img_from_file_success() {
//         let path = PathBuf::from("tests/assets/test.png");
//         let img = Img::from_file(&path).expect("Image should open successfully");

//         assert_eq!(img.format, ImageFormat::Png);

//         let expected_ratio = img.width as f32 / img.height as f32;
//         assert!((img.aspect_ratio - expected_ratio).abs() < 0.01);
//     }

//     #[test]
//     fn test_img_from_file_multiple_formats() {
//         let cases = [
//             ("test.png", ImageFormat::Png),
//             ("test.jpg", ImageFormat::Jpeg),
//             ("test.webp", ImageFormat::WebP),
//         ];

//         for (file, fmt) in cases {
//             let path = PathBuf::from(format!("tests/assets/{}", file));
//             let img = Img::from_file(&path).expect(&format!("Should open {}", file));
//             assert_eq!(img.format, fmt);
//         }
//     }

//     #[test]
//     fn test_img_from_file_nonexistent_path() {
//         let path = PathBuf::from("tests/assets/does_not_exist.png");
//         let err = Img::from_file(&path).unwrap_err();
//         match err {
//             ImgError::Validation(ValidationError::PathNotFound(p)) => assert_eq!(p, path),
//             _ => panic!("Expected ImgError::Validation(PathNotFound), got {:?}", err),
//         }
//     }

//     #[test]
//     fn test_img_from_file_directory() {
//         let dir = Path::new("tests/assets");
//         let err = Img::from_file(dir).unwrap_err();
//         match err {
//             ImgError::Validation(ValidationError::NotAFile(p)) => assert_eq!(p, dir.to_path_buf()),
//             _ => panic!("Expected ImgError::Validation(NotAFile), got {:?}", err),
//         }
//     }

//     #[test]
//     fn test_img_from_file_missing_extension() {
//         let tmp = TempDir::new().unwrap();
//         let file = tmp.path().join("no_ext");
//         File::create(&file).unwrap();

//         let err = Img::from_file(&file).unwrap_err();
//         match err {
//             ImgError::Validation(ValidationError::MissingExtension(p)) => assert_eq!(p, file),
//             _ => panic!(
//                 "Expected ImgError::Validation(MissingExtension), got {:?}",
//                 err
//             ),
//         }
//     }

//     #[test]
//     fn test_img_from_file_unsupported_extension() {
//         let tmp = TempDir::new().unwrap();
//         let file = tmp.path().join("foo.txt");
//         File::create(&file).unwrap();

//         let err = Img::from_file(&file).unwrap_err();
//         match err {
//             ImgError::Validation(ValidationError::NotAnImageFile(p)) => assert_eq!(p, file),
//             _ => panic!(
//                 "Expected ImgError::Validation(NotAnImageFile), got {:?}",
//                 err
//             ),
//         }
//     }
// }
