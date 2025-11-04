use {
    crate::{
        image::{utils::decode, ImageConfig, ImageOrigin},
        ErrorKind, Format, Image, Result, WithOrigin,
    },
    std::{
        cell::RefCell,
        io::{BufReader, Cursor},
    },
};

impl Image {
    pub fn from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self> {
        (|| -> Result<Self> {
            let mut reader = BufReader::new(Cursor::new(bytes));
            let format = Format::guess_from_reader(&mut reader)?.ok_or(ErrorKind::UnknownFormat)?;
            let decoded = decode::from_reader(&mut reader, &format)?;

            Ok(Self {
                origin: ImageOrigin::Bytes,
                decoded: RefCell::new(decoded),
                config: ImageConfig::default(),
                format: Some(format),
            })
        })()
        .with_origin(ImageOrigin::Bytes)
    }
}

// #[cfg(test)]
// mod tests {
//     use {
//         super::*,
//         crate::{test_utils::encoded_bytes, ImageFormat},
//         strum::IntoEnumIterator,
//     };

//     #[test]
//     fn from_bytes_ok() -> Result<()> {
//         for format in ImageFormat::iter() {
//             let bytes = encoded_bytes(format);

//             let img = Image::from_bytes(&bytes)?;

//             match img.src {
//                 ImageSrc::Bytes => {}
//                 _ => panic!("expected ImageSrc::Bytes"),
//             }

//             {
//                 let data = img.data.borrow();
//                 match &*data {
//                     ImageData::EncodedBytes(b) => assert_eq!(b, &bytes),
//                     _ => panic!("expected ImageData::EncodedBytes"),
//                 }
//             }
//         }

//         Ok(())
//     }

//     #[test]
//     fn from_bytes_rejects_empty_input() {
//         let res = Image::from_bytes(&[]);
//         assert!(res.is_err(), "expected error for empty input");
//     }

//     #[test]
//     fn from_bytes_rejects_garbage_input() {
//         let garbage = b"not an image at all";
//         let res = Image::from_bytes(garbage);
//         assert!(res.is_err(), "expected error for invalid bytes");
//     }
// }
