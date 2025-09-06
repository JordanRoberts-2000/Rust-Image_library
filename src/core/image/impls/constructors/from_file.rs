use {
    crate::{
        image::{enums::ImageSrc, utils, ImageConfig},
        Image, ImageMetadata, Result, ValidationError,
    },
    fs_ext::fsx,
    std::{num::NonZeroU32, path::Path},
};

impl Image {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        fsx::file::assert_exists(path)?;

        let metadata = ImageMetadata::from_path(path)?;
        let (file_name, parent_dir) = utils::file_info(path);

        Ok(Self {
            src: ImageSrc::File(path.to_path_buf()),
            data: None,
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
        super::*,
        crate::test_utils::write_png,
        std::{io::Write as _, path::PathBuf},
        tempfile::tempdir,
    };

    #[test]
    fn from_file_ok() -> Result<()> {
        let dir = tempdir().expect("failed to create tempdir");
        let path = dir.path().join("pixel.png");

        write_png(&path);

        let img = Image::from_file(&path)?;

        match &img.src {
            ImageSrc::File(p) => assert_eq!(p, &path),
            _ => panic!("expected ImageSrc::File"),
        }

        assert!(img.data.is_none(), "from_file should not eagerly populate pixel data");

        Ok(())
    }

    #[test]
    fn from_file_rejects_missing_path() {
        let dir = tempdir().expect("failed to create tempdir");
        let missing: PathBuf = dir.path().join("does_not_exist.png");
        Image::from_file(&missing).expect_err("expected error for missing path");
    }

    #[test]
    fn from_file_rejects_directory_path() {
        let dir = tempdir().expect("failed to create tempdir");

        Image::from_file(dir.path()).expect_err("expected error when passing a directory");
    }

    #[test]
    fn from_file_rejects_non_image_file() {
        let dir = tempdir().expect("failed to create tempdir");
        let bogus = dir.path().join("not_an_image.txt");
        {
            let mut f = std::fs::File::create(&bogus).expect("failed to create temp file");
            writeln!(f, "this is not an image").unwrap();
        }
        Image::from_file(&bogus).expect_err("expected error for non-image file");
    }
}
