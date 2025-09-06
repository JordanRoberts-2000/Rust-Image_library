use {
    crate::{
        image::{
            enums::{ImageData, ImageSrc},
            ImageConfig,
        },
        Image, ImageError, ImageMetadata, Result, ValidationError,
    },
    base64::Engine,
    std::num::NonZeroU32,
};

impl Image {
    pub fn from_base64(base_64: impl AsRef<str>) -> Result<Self> {
        let base_64 = base_64.as_ref();

        let bytes = base64::engine::general_purpose::STANDARD
            .decode(base_64)
            .map_err(|e| ImageError::Base64DecodeFailed(e, base_64.to_string()))?;

        let metadata = ImageMetadata::from_bytes(&bytes)?;

        Ok(Self {
            src: ImageSrc::Base64,
            data: Some(ImageData::EncodedBytes(bytes)),
            config: ImageConfig::default(),
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
        crate::{
            image::enums::{ImageData, ImageSrc},
            test_utils::png_bytes,
        },
        base64::engine::general_purpose::STANDARD,
    };

    #[test]
    fn from_base64_ok() -> Result<()> {
        let bytes = png_bytes();
        let b64 = STANDARD.encode(&bytes);

        let img = Image::from_base64(&b64)?;

        match img.src {
            ImageSrc::Base64 => {}
            _ => panic!("expected ImageSrc::Base64"),
        }

        match img.data {
            Some(ImageData::EncodedBytes(ref b)) => assert_eq!(b, &bytes),
            _ => panic!("expected Some(EncodedBytes)"),
        }

        Ok(())
    }

    #[test]
    fn from_base64_rejects_invalid_base64() {
        let bad = "not base64 !!!";
        match Image::from_base64(bad) {
            Err(ImageError::Base64DecodeFailed(_, s)) => assert_eq!(s, bad),
            Err(e) => panic!("expected Base64DecodeFailed, got {e:?}"),
            Ok(_) => panic!("expected error"),
        }
    }

    #[test]
    fn from_base64_rejects_non_image_payload() {
        let payload = b"this is not an image";
        let b64 = STANDARD.encode(payload);

        let res = Image::from_base64(&b64);
        assert!(res.is_err(), "expected error when base64 decodes to non-image bytes");
    }
}
