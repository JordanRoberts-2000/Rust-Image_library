use std::path::{Path, PathBuf};

use crate::{
    constants::DEFAULT_IMAGE_FILE_NAME, AsyncImage, AsyncImageService, AsyncMetadataRepo,
    AsyncValidationRepo, ImageConfig, ImageData, ImageSrc, Result,
};

impl AsyncImage {
    pub async fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        Self::from_file_internal(path, AsyncImageService::new()).await
    }

    async fn from_file_internal<M: AsyncMetadataRepo, V: AsyncValidationRepo>(
        path: &Path,
        service: AsyncImageService<M, V>,
    ) -> Result<Self> {
        service.validation.ensure_existing_image_file(path).await?;

        let (format, width, height) = service.metadata.from_path(path).await?;
        let (file_name, output_dir) = Self::file_info_from_path(path);

        Ok(Self {
            src: ImageSrc::File(path.to_path_buf()),
            data: ImageData::File(path.to_path_buf()),
            config: ImageConfig {
                file_name,
                output_dir,
                ..Default::default()
            },
            height,
            width,
            aspect_ratio: width as f32 / height as f32,
            format,
        })
    }

    fn file_info_from_path(path: &Path) -> (String, PathBuf) {
        let file_name = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or(DEFAULT_IMAGE_FILE_NAME)
            .to_string();

        let output_dir = path
            .parent()
            .map_or_else(|| PathBuf::from("."), |p| p.to_path_buf());

        (file_name, output_dir)
    }
}

// #[cfg(test)]
// mod tests {
//     use mockall::predicate;

//     use crate::{BlockingImage, MockMetadataRepo, MockValidationRepo};

//     #[test]
//     fn test_from_file_internal_with_mocks() {
//         use crate::{ImageData, ImageFormat, ImageService, ImageSrc};
//         use std::path::PathBuf;

//         let path = PathBuf::from("fake_image.png");

//         let mut mock_validation = MockValidationRepo::new();
//         mock_validation
//             .expect_ensure_existing_image_file()
//             .with(predicate::eq(path.clone()))
//             .returning(|_| Ok(()));

//         let mut mock_metadata = MockMetadataRepo::new();
//         mock_metadata
//             .expect_from_path()
//             .with(predicate::eq(path.clone()))
//             .returning(|_| Ok((ImageFormat::Png, 800, 600)));

//         let service = ImageService {
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
