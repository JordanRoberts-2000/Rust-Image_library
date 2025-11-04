use crate::{pixels::Rgba, ErrorKind, Image, Result, WithOrigin};

impl Image {
    pub fn blurhash(&self) -> Result<String> {
        let decoded = self.decoded();
        let (w, h) = decoded.dimensions();

        blurhash::encode(4, 3, w, h, &decoded.as_bytes::<Rgba<u8>>())
            .map_err(ErrorKind::BlurHash)
            .with_origin(self.origin())
    }
}
