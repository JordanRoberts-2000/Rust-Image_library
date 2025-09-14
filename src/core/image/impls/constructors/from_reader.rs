use {
    crate::{
        image::{
            enums::{ImageData, ImageSrc},
            ImageConfig,
        },
        Image, ImageMetadata, Result,
    },
    std::{cell::RefCell, io::Read},
};

impl Image {
    pub fn from_reader(reader: &mut (impl Read + ?Sized)) -> Result<Self> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes)?;

        let metadata = ImageMetadata::from_bytes(&bytes)?;

        Ok(Self {
            src: ImageSrc::Reader,
            data: RefCell::new(ImageData::EncodedBytes(bytes)),
            config: ImageConfig::default(),
            metadata,
        })
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::test_utils::png_bytes,
        std::io::{Cursor, Read},
    };

    #[test]
    fn from_reader_ok() -> Result<()> {
        let bytes = png_bytes();

        let mut cur = Cursor::new(bytes.clone());
        let img = Image::from_reader(&mut cur)?;

        match img.src {
            ImageSrc::Reader => {}
            _ => panic!("expected ImageSrc::Reader"),
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
    fn from_reader_accepts_non_static_reader_slice() -> Result<()> {
        let bytes = png_bytes();
        let slice = &bytes[..]; // non-'static borrow
        let mut cur = Cursor::new(slice); // Cursor<&[u8]> implements Read
        let _ = Image::from_reader(&mut cur)?;
        Ok(())
    }

    #[test]
    fn from_reader_accepts_dyn_read_trait_object() -> Result<()> {
        let bytes = png_bytes();
        let mut cur = Cursor::new(bytes.clone());
        let r: &mut dyn Read = &mut cur; // trait object (unsized)
        Image::from_reader(r)?;
        Ok(())
    }

    #[test]
    fn from_reader_rejects_empty_input() {
        let mut cur = Cursor::new(Vec::<u8>::new());
        let res = Image::from_reader(&mut cur);
        assert!(res.is_err(), "expected error for empty reader");
    }

    #[test]
    fn from_reader_rejects_non_image_payload() {
        let mut cur = Cursor::new(b"not an image".to_vec());
        let res = Image::from_reader(&mut cur);
        assert!(res.is_err(), "expected error for non-image bytes");
    }
}
