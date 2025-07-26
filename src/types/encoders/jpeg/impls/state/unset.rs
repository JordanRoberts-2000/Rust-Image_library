use {
    crate::{
        encoders::{
            jpeg::{Bytes, Raw, Reader, Unset},
            JpegColorType, JpegEncoder,
        },
        ImageFormat,
    },
    std::io::Read,
};

impl JpegEncoder<Unset> {
    pub fn with_quality(mut self, quality: u8) -> Self {
        self.quality = quality.clamp(1, 100);
        self
    }

    pub fn with_color_type(mut self, color_type: JpegColorType) -> Self {
        self.color_type = Some(color_type);
        self
    }

    pub fn set_progressive(mut self, progressive: bool) -> Self {
        self.progressive = progressive;
        self
    }

    pub fn from_raw_pixels<'a>(
        self, pixels: &'a [u8], width: u32, height: u32,
    ) -> JpegEncoder<Raw<'a>> {
        JpegEncoder {
            quality: self.quality,
            color_type: self.color_type,
            progressive: self.progressive,
            input: Raw { width, height, bytes: pixels },
        }
    }

    pub fn from_encoded_bytes<'a>(self, bytes: &'a [u8]) -> JpegEncoder<Bytes<'a>> {
        JpegEncoder {
            quality: self.quality,
            color_type: self.color_type,
            progressive: self.progressive,
            input: Bytes { bytes, format: None },
        }
    }

    pub fn from_encoded_bytes_with_format<'a>(
        self, bytes: &'a [u8], format: ImageFormat,
    ) -> JpegEncoder<Bytes<'a>> {
        JpegEncoder {
            quality: self.quality,
            color_type: self.color_type,
            progressive: self.progressive,
            input: Bytes { bytes, format: Some(format) },
        }
    }

    pub fn from_encoded_reader<R: Read>(self, reader: R) -> JpegEncoder<Reader<R>> {
        JpegEncoder {
            quality: self.quality,
            color_type: self.color_type,
            progressive: self.progressive,
            input: Reader { reader, format: None },
        }
    }

    pub fn from_encoded_reader_with_format<R: Read>(
        self, reader: R, format: ImageFormat,
    ) -> JpegEncoder<Reader<R>> {
        JpegEncoder {
            quality: self.quality,
            color_type: self.color_type,
            progressive: self.progressive,
            input: Reader { reader, format: Some(format) },
        }
    }
}
