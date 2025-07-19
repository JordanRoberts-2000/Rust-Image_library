use {
    crate::{
        encoders::{
            utils::{quantize_image_to_8bit, try_strip_alpha_if_unused, validate_dimensions},
            PngEncoder,
        },
        EncodingError, ImageError, Result,
    },
    image::{
        codecs::png::PngEncoder as Encoder, load_from_memory, ColorType, GenericImageView,
        ImageEncoder,
    },
    std::io::Write,
};

impl PngEncoder {
    pub(super) fn encode(&self, bytes: &[u8], writer: impl Write) -> Result<()> {
        let mut img = load_from_memory(bytes).map_err(ImageError::DecodeFromMemory)?;

        let (width, height) = img.dimensions();
        validate_dimensions(width, height)?;

        let mut color_type: ColorType = match &self.color_type {
            Some(ct) => ct.into(),
            None => img.color(),
        };

        if self.quantize_to_8bit {
            if let Some(downsized) = quantize_image_to_8bit(&img) {
                img = downsized;
                color_type = img.color();
            }
        }

        if self.strip_unused_transparency && color_type.has_alpha() {
            if let Some(stripped) = try_strip_alpha_if_unused(&img) {
                img = stripped;
                color_type = img.color();
            }
        }

        let encoder = Encoder::new_with_quality(writer, self.compression, self.filter);
        encoder
            .write_image(&img.into_bytes(), width, height, color_type.into())
            .map_err(EncodingError::PngEncoding)?;

        Ok(())
    }
}
