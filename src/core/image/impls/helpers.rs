use {
    crate::{
        encoding::{AvifColorType, ColorType, JpegColorType, PngColorType, WebpColorType},
        image::{types::ResolvedColorType, utils::alpha_is_unused},
        Image, ImageFormat, Result,
    },
    image::DynamicImage,
};

impl Image {
    pub(crate) fn resolve_color_type(
        &self, img: &DynamicImage, format: ImageFormat,
    ) -> Result<ResolvedColorType> {
        let base: ColorType = img.color().try_into()?;

        match format {
            ImageFormat::Png => {
                let mut ct = PngColorType::from(base);

                if self.config.minimize_bit_depth {
                    ct = ct.to_minimal_bit_depth();
                }

                if self.config.remove_unused_transparency
                    && base.has_alpha()
                    && ct.has_alpha()
                    && alpha_is_unused(img)
                {
                    ct = ct.remove_alpha();
                }
                Ok(ResolvedColorType::Png(ct))
            }

            ImageFormat::Jpeg => {
                Ok(ResolvedColorType::Jpeg(JpegColorType::try_from(base).unwrap_or_default()))
            }

            ImageFormat::WebP => {
                let mut ct = WebpColorType::try_from(base).unwrap_or_default();
                if self.config.remove_unused_transparency
                    && base.has_alpha()
                    && ct.has_alpha()
                    && alpha_is_unused(img)
                {
                    ct = ct.remove_alpha();
                }
                Ok(ResolvedColorType::Webp(ct))
            }

            ImageFormat::Avif => {
                let mut ct = AvifColorType::try_from(base).unwrap_or_default();
                if self.config.remove_unused_transparency
                    && base.has_alpha()
                    && ct.has_alpha()
                    && alpha_is_unused(img)
                {
                    ct = ct.remove_alpha();
                }
                Ok(ResolvedColorType::Avif(ct))
            }
        }
    }
}
