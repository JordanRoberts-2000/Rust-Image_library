use {
    crate::{
        blocking::{
            dependencies::ImageService,
            traits::{FsRepoOps, ImageServiceOps, MetadataOps},
        },
        image::{
            blocking::{Image, ImageData},
            enums::ImageSrc,
            utils::file_info,
            ImageConfig,
        },
        Result, ValidationError,
    },
    std::{cell::RefCell, num::NonZeroU32, path::Path, rc::Rc},
};

impl Image {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        Self::from_file_internal(path, &ImageService::default())
    }

    fn from_file_internal(path: &Path, service: &impl ImageServiceOps) -> Result<Self> {
        service.fs().check_existing_file(path)?;

        let metadata = service.metadata().from_path(path)?;
        let (file_name, parent_dir) = file_info(path);

        Ok(Self {
            src: ImageSrc::File(path.to_path_buf()),
            data: Rc::new(RefCell::new(ImageData::File(path.to_path_buf()))),
            config: ImageConfig { file_name, output_dir: parent_dir, ..Default::default() },
            height: NonZeroU32::new(metadata.height).ok_or(ValidationError::InvalidHeight)?,
            width: NonZeroU32::new(metadata.width).ok_or(ValidationError::InvalidWidth)?,
            format: metadata.format,
        })
    }
}

#[cfg(test)]
mod tests {
    use {
        crate::{
            blocking::{
                dependencies::MockImageService,
                traits::{MockFsRepoOps, MockMetadataOps},
                Image,
            },
            image::{blocking::ImageData, enums::ImageSrc},
            ImageError, ImageFormat, ImageMetadata, ValidationError,
        },
        std::{cell::RefCell, path::PathBuf, rc::Rc},
    };

    #[test]
    fn test_from_file_internal_success() {
        let path = PathBuf::from("test_image.jpg");

        let mut fs_mock = MockFsRepoOps::new();
        fs_mock.expect_check_existing_file().returning(|_| Ok(()));

        let mut metadata_mock = MockMetadataOps::new();
        metadata_mock
            .expect_from_path()
            .returning(|_| Ok(ImageMetadata::new(800, 600, ImageFormat::Png)));

        let mock_deps =
            MockImageService { fs: fs_mock, metadata: metadata_mock, ..Default::default() };

        let image = Image::from_file_internal(&path, &mock_deps).unwrap();

        assert_eq!(image.width(), 1920);
        assert_eq!(image.height(), 1080);
        assert_eq!(image.aspect_ratio(), 1920.0 / 1080.0);
        assert_eq!(image.format, ImageFormat::Jpeg);
        assert_eq!(image.src, ImageSrc::File(path.clone()));
        assert_eq!(image.data, Rc::new(RefCell::new(ImageData::File(path.clone()))));
    }

    #[test]
    fn test_from_file_internal_fails_when_file_missing() {
        let path = PathBuf::from("missing.jpg");

        let mut fs_mock = MockFsRepoOps::new();
        fs_mock.expect_check_existing_file().returning(|path| {
            Err(ImageError::Validation(ValidationError::PathNotFound(path.to_path_buf())))
        });

        let mock_deps = MockImageService { fs: fs_mock, ..Default::default() };

        let result = Image::from_file_internal(&path, &mock_deps);

        assert!(matches!(result, Err(ImageError::Validation(ValidationError::PathNotFound(_)))));
    }

    #[test]
    fn test_from_file_internal_fails_when_metadata_fails() {
        let path = PathBuf::from("corrupt.jpg");

        let mut fs_mock = MockFsRepoOps::new();
        fs_mock.expect_check_existing_file().returning(|_| Ok(()));

        let mut metadata_mock = MockMetadataOps::new();
        metadata_mock.expect_from_path().returning(|_| Err(ImageError::UnknownFormat));

        let mock_deps =
            MockImageService { fs: fs_mock, metadata: metadata_mock, ..Default::default() };

        let result = Image::from_file_internal(&path, &mock_deps);

        assert!(matches!(result, Err(ImageError::UnknownFormat)));
    }
}
