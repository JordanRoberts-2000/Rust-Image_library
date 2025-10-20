use {
    crate::{
        image::{utils::decode, ImageConfig},
        ErrorKind, Format, Image, ImageSrc, Result, WithSrc,
    },
    fs_ext::PathExt,
    std::{cell::RefCell, fs::File, io::BufReader, path::Path},
};

impl Image {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let src: ImageSrc = path.into();

        let res: Result<Self> = (|| {
            let mut reader = BufReader::new(File::open(path)?);
            let format = Format::guesser()
                .extension_hint(path.extension())
                .from_reader(&mut reader)?
                .ok_or(ErrorKind::UnknownFormat)?;
            let decoded = decode::from_reader(&mut reader, &format)?;

            let file_name = path.utf8_stem()?.to_owned();
            let parent_dir = path.parent_or_current();

            Ok(Self {
                src: src.clone(),
                decoded: RefCell::new(decoded),
                config: ImageConfig { file_name, output_dir: parent_dir, ..Default::default() },
                format: Some(format),
            })
        })();

        res.with_src(src)
    }
}

// #[cfg(test)]
// mod tests {
//     use {
//         super::*,
//         crate::{test_utils::image_file, ImageFormat},
//         std::{io::Write as _, path::PathBuf},
//         strum::IntoEnumIterator,
//         tempfile::{tempdir, TempDir},
//     };

//     #[test]
//     fn from_file_ok() -> Result<()> {
//         let temp_dir = TempDir::new().unwrap();

//         for format in ImageFormat::iter() {
//             let path = image_file(&temp_dir, format);

//             let img = Image::from_file(&path)?;

//             match &img.src {
//                 ImageSrc::File(p) => assert_eq!(p, &path),
//                 _ => panic!("expected ImageSrc::File"),
//             }

//             {
//                 let data = img.data.borrow();
//                 match &*data {
//                     ImageData::File(p) => assert_eq!(p, &path),
//                     _ => panic!("expected ImageData::File"),
//                 }
//             }
//         }

//         Ok(())
//     }

//     #[test]
//     fn from_file_rejects_missing_path() {
//         let dir = tempdir().expect("failed to create tempdir");
//         let missing: PathBuf = dir.path().join("does_not_exist.png");
//         Image::from_file(&missing).expect_err("expected error for missing path");
//     }

//     #[test]
//     fn from_file_rejects_directory_path() {
//         let dir = tempdir().expect("failed to create tempdir");

//         Image::from_file(dir.path()).expect_err("expected error when passing a directory");
//     }

//     #[test]
//     fn from_file_rejects_non_image_file() {
//         let dir = tempdir().expect("failed to create tempdir");
//         let bogus = dir.path().join("not_an_image.txt");
//         {
//             let mut f = std::fs::File::create(&bogus).expect("failed to create temp file");
//             writeln!(f, "this is not an image").unwrap();
//         }
//         Image::from_file(&bogus).expect_err("expected error for non-image file");
//     }
// }
