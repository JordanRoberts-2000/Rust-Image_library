use {
    crate::{image::ImageConfig, utils::decode, Image, ImageMetadata, ImageSrc, Result, WithSrc},
    image::GenericImageView,
    std::cell::RefCell,
};

impl Image {
    pub fn from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self> {
        let (decoded, format) = decode::from_bytes(bytes).with_src(ImageSrc::Bytes)?;
        let (w, h) = decoded.dimensions();

        Ok(Self {
            src: ImageSrc::Bytes,
            decoded: RefCell::new(decoded),
            config: ImageConfig::default(),
            metadata: ImageMetadata::new(w, h, format).with_src(ImageSrc::Bytes)?,
        })
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
