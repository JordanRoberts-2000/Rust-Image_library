use {
    crate::{
        encoders::{
            utils::{try_strip_alpha_if_unused, validate_dimensions},
            AvifEncoder,
        },
        EncodingError, ImageError, Result,
    },
    image::{load_from_memory, ColorType, GenericImageView},
    ravif::{Encoder, Img, RGB8, RGBA8},
    std::io::Write,
};

impl AvifEncoder {
    pub(super) fn encode(&self, bytes: &[u8], mut writer: impl Write) -> Result<()> {
        let mut img = load_from_memory(bytes).map_err(ImageError::DecodeFromMemory)?;

        let (width, height) = img.dimensions();
        validate_dimensions(width, height)?;

        let mut color_type: ColorType = match &self.color_type {
            Some(ct) => ct.into(),
            None => img.color(),
        };

        if self.strip_unused_transparency && color_type.has_alpha() {
            if let Some(stripped) = try_strip_alpha_if_unused(&img) {
                img = stripped;
                color_type = img.color();
            }
        }

        let encoder = Encoder::new()
            .with_quality(self.quality as f32)
            .with_speed(self.speed)
            .with_alpha_quality(self.alpha_quality as f32);

        let encoded = if color_type.has_alpha() {
            let rgba_data: Vec<RGBA8> = img
                .as_bytes()
                .chunks_exact(4)
                .map(|chunk| RGBA8::new(chunk[0], chunk[1], chunk[2], chunk[3]))
                .collect();

            let img_rgba = Img::new(rgba_data.as_slice(), width as usize, height as usize);

            encoder.encode_rgba(img_rgba).map_err(|err| EncodingError::AvifEncoding { err })?
        } else {
            let rgb_data: Vec<RGB8> = img
                .as_bytes()
                .chunks_exact(3)
                .map(|chunk| RGB8::new(chunk[0], chunk[1], chunk[2]))
                .collect();

            let img_rgb = Img::new(rgb_data.as_slice(), width as usize, height as usize);

            encoder.encode_rgb(img_rgb).map_err(|err| EncodingError::AvifEncoding { err })?
        };

        writer.write_all(&encoded.avif_file).map_err(crate::IoError::WriteAll)?;

        Ok(())
    }
}
