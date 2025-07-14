use {
    crate::{blocking::Image, ColorType, ImageError, IoError, Result},
    std::io::Read,
};

impl Image {
    pub fn from_raw_reader(
        reader: impl Read,
        width: u32,
        height: u32,
        color_type: ColorType,
    ) -> Result<Self> {
        let expected_len = width as usize * height as usize * color_type.bytes_per_pixel();
        let mut pixels = Vec::with_capacity(expected_len);

        reader
            .take(expected_len as u64)
            .read_to_end(&mut pixels)
            .map_err(IoError::ReadStream)?;

        if pixels.len() != expected_len {
            return Err(ImageError::InvalidBuffer(color_type));
        }

        Self::from_raw_pixels(pixels, width, height, color_type)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        blocking::Image,
        image::{blocking::ImageData, enums::ImageSrc},
        ColorType, ImageError, IoError,
    };

    #[test]
    fn test_from_raw_reader_success() {
        let width = 2;
        let height = 2;
        let color_type = ColorType::Rgb8;
        let pixel_data = vec![255u8; width * height * 3]; // 3 bytes per pixel for RGB

        let reader = std::io::Cursor::new(pixel_data.clone());

        let result =
            Image::from_raw_reader(reader, width as u32, height as u32, color_type.clone());

        assert!(result.is_ok());

        let image = result.unwrap();

        assert_eq!(image.width(), width as u32);
        assert_eq!(image.height(), height as u32);
        assert_eq!(image.src, ImageSrc::RawPixels);
        assert!(matches!(image.data, ImageData::Decoded(_)));
    }

    #[test]
    fn test_from_raw_reader_too_few_bytes() {
        let width = 2;
        let height = 2;
        let color_type = ColorType::Rgba8;
        let expected_len = width * height * 4;

        let pixel_data = vec![0u8; expected_len - 1]; // One byte too short
        let reader = std::io::Cursor::new(pixel_data);

        let result =
            Image::from_raw_reader(reader, width as u32, height as u32, color_type.clone());

        assert!(matches!(result, Err(ImageError::InvalidBuffer(ct)) if ct == color_type));
    }

    struct FailingReader;

    impl std::io::Read for FailingReader {
        fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "read failure",
            ))
        }
    }

    #[test]
    fn test_from_raw_reader_io_error() {
        let reader = FailingReader;
        let color_type = ColorType::L8;

        let result = Image::from_raw_reader(reader, 2, 2, color_type);

        assert!(matches!(
            result,
            Err(ImageError::Io(IoError::ReadStream(_)))
        ));
    }
}
