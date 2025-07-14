use std::io::Write;

use crate::{
    blocking::Image,
    image::utils::{encode_webp_data, resolve_webp_config},
    Result,
};

impl Image {
    pub fn encode_webp(&mut self, writer: impl Write) -> Result<()> {
        let rgba_image = self.get_decoded()?.to_rgba8();
        let (width, height) = rgba_image.dimensions();
        let id = self.describe_source();

        let (lossless, quality) = resolve_webp_config(&self.config);
        let webp_data = encode_webp_data(&rgba_image, width, height, lossless, quality, &id)?;

        Self::write_encoded(writer, &webp_data, id)
    }
}
