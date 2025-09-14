use {
    crate::{
        image::{
            enums::{ImageData, ImageSrc},
            ImageConfig,
        },
        Image, ImageMetadata, Result,
    },
    std::cell::RefCell,
};

impl Image {
    pub fn from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self> {
        let bytes = bytes.as_ref();
        let metadata = ImageMetadata::from_bytes(bytes)?;

        Ok(Self {
            src: ImageSrc::Bytes,
            data: RefCell::new(ImageData::EncodedBytes(bytes.to_vec())),
            config: ImageConfig::default(),
            metadata,
        })
    }
}

#[cfg(test)]
mod tests {
    use {super::*, crate::test_utils::png_bytes};

    #[test]
    fn from_bytes_ok() -> Result<()> {
        let bytes = png_bytes();

        let img = Image::from_bytes(&bytes)?;

        match img.src {
            ImageSrc::Bytes => {}
            _ => panic!("expected ImageSrc::Bytes"),
        }

        {
            let data = img.data.borrow();
            match &*data {
                ImageData::EncodedBytes(b) => assert_eq!(b, &bytes),
                _ => panic!("expected ImageData::EncodedBytes"),
            }
        }

        Ok(())
    }

    #[test]
    fn from_bytes_rejects_empty_input() {
        let res = Image::from_bytes(&[]);
        assert!(res.is_err(), "expected error for empty input");
    }

    #[test]
    fn from_bytes_rejects_garbage_input() {
        let garbage = b"not an image at all";
        let res = Image::from_bytes(garbage);
        assert!(res.is_err(), "expected error for invalid bytes");
    }
}
