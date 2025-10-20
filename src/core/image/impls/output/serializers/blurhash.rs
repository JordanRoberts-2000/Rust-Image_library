use {
    crate::{ErrorKind, Image, Result, WithSrc},
    image::GenericImageView,
};

impl Image {
    pub fn blurhash(&self) -> Result<String> {
        let decoded = self.processed_decode();
        let img = decoded.get_static()?;
        let (w, h) = img.dimensions();

        blurhash::encode(4, 3, w, h, img.to_rgba8().as_raw())
            .map_err(ErrorKind::BlurHash)
            .with_src(self.src())
    }
}
