use {
    crate::{
        image::{
            blocking::{
                dependencies::ImageDeps,
                traits::{FsOps, ImageDepsOps, MetadataOps},
                Image,
            },
            enums::{ImageData, ImageSrc},
            helpers::file_info,
            ImageConfig,
        },
        Result,
    },
    std::path::Path,
};

impl Image {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        Self::from_file_internal(path, &ImageDeps::default())
    }

    fn from_file_internal(path: &Path, image_deps: &impl ImageDepsOps) -> Result<Self> {
        image_deps.fs().ensure_existing_file(path)?;

        let (format, width, height) = image_deps.metadata().from_path(path)?;
        let (file_name, parent_dir) = file_info(path);

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
//     use mockall::predicate;

//     use crate::{
//         BlockingImage, ImageData, ImageFormat, ImageSrc, MockSyncMetadataRepo,
//         MockSyncValidationRepo, SyncImageService,
//     };

//     #[test]
//     fn test_from_file_internal_with_mocks() {
//         use std::path::PathBuf;

//         let path = PathBuf::from("fake_image.png");

//         let mut mock_validation = MockSyncValidationRepo::new();
//         mock_validation
//             .expect_ensure_existing_image_file()
//             .with(predicate::eq(path.clone()))
//             .returning(|_| Ok(()));

//         let mut mock_metadata = MockSyncMetadataRepo::new();
//         mock_metadata
//             .expect_from_path()
//             .with(predicate::eq(path.clone()))
//             .returning(|_| Ok((ImageFormat::Png, 800, 600)));

//         let service = SyncImageService {
//             validation: mock_validation,
//             metadata: mock_metadata,
//         };

//         let result = BlockingImage::from_file_internal(&path, service).unwrap();

//         assert_eq!(result.height, 600);
//         assert_eq!(result.width, 800);
//         assert_eq!(result.format, ImageFormat::Png);
//         assert_eq!(result.src, ImageSrc::File(path.clone()));
//         assert_eq!(result.data, ImageData::File(path.clone()));
//     }
// }

// #[cfg(test)]
// mod tests {
//     use {mockall::predicate::*, std::path::Path};

//     use crate::{blocking::MockImageOps, MockRepo, SyncImageOpsInternal};

//     #[test]
//     fn test_mock_from_file_success() {
//         let ctx = MockImageOps::<MockRepo>::from_file_internal_context();

//         ctx.expect()
//             .with(eq(Path::new("test.jpg")), always())
//             .times(1)
//             .returning(|_, _| Ok(MockImageOps::new()));

//         let repo = MockRepo::new();
//         let result = MockImageOps::from_file_internal(Path::new("test.jpg"), repo);
//         assert!(result.is_ok());
//     }
// }
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
