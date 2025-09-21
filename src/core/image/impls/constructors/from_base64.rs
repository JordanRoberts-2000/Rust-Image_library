use {
    crate::{
        image::{ImageConfig, ImageData, ImageMetadata},
        ErrorKind, Image, ImageSrc, InnerError, Result, ResultCtx,
    },
    base64::Engine,
    std::cell::RefCell,
};

impl Image {
    pub fn from_base64(base_64: impl AsRef<str>) -> Result<Self> {
        let base_64 = base_64.as_ref();

        let preview = base_64.chars().take(10).collect();
        let src = ImageSrc::Base64(preview);

        let bytes = base64::engine::general_purpose::STANDARD
            .decode(base_64)
            .map_err(|e| InnerError::Base64DecodeFailed(e, base_64.to_string()))
            .ctx(ErrorKind::Decode, None)?;

        let metadata =
            ImageMetadata::from_bytes(&bytes).ctx(ErrorKind::ReadMetadata, Some(&src.clone()))?;

        Ok(Self {
            src,
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
        crate::{image::ImageData, test_utils::png_bytes, ImageSrc},
        base64::engine::general_purpose::STANDARD,
    };

    #[test]
    fn from_base64_ok() -> Result<()> {
        let bytes = png_bytes();
        let b64 = STANDARD.encode(&bytes);

        let img = Image::from_base64(&b64)?;

        match img.src {
            ImageSrc::Base64(_) => {}
            _ => panic!("expected ImageSrc::Base64"),
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
    fn from_base64_rejects_invalid_base64() {
        let bad = "not base64 !!!";

        match Image::from_base64(bad) {
            Err(err) => {
                assert_eq!(err.kind(), ErrorKind::Decode, "wrong error kind");
                match err.inner() {
                    InnerError::Base64DecodeFailed(_, s) => assert_eq!(s, bad),
                    other => panic!("expected Base64DecodeFailed, got {other:?}"),
                }
            }
            Ok(_) => panic!("expected error"),
        }
    }

    #[test]
    fn preview_stores_first_10_chars() -> Result<()> {
        let bytes = png_bytes();
        let b64 = STANDARD.encode(&bytes);
        let img = Image::from_base64(&b64)?;

        match img.src {
            ImageSrc::Base64(preview) => {
                assert_eq!(preview.len(), 10);
                assert_eq!(preview, &b64[..10]);
            }
            _ => panic!("expected ImageSrc::Base64"),
        }

        Ok(())
    }

    #[test]
    fn from_base64_rejects_non_image_payload() {
        let payload = b"this is not an image";
        let b64 = STANDARD.encode(payload);

        let res = Image::from_base64(&b64);
        assert!(res.is_err(), "expected error when base64 decodes to non-image bytes");
    }
}
