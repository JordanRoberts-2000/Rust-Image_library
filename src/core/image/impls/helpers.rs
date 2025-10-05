use {
    crate::{
        image::utils::alpha_is_unused, AvifColorType, ColorType, Image, ImageFormat, JpegColorType,
        PngColorType, Result, WebPColorType, WithSrc,
    },
    image::DynamicImage,
};

impl Image {
    pub(crate) fn resolve_color_type(&self, img: &DynamicImage) -> Result<ColorType> {
        let color_type: ColorType = img.color().try_into().with_src(self.error_src())?;

        let mut output: ColorType = match self.format() {
            ImageFormat::WebP => WebPColorType::try_from(color_type).unwrap_or_default().into(),
            ImageFormat::Png => {
                let color_type = PngColorType::from(color_type);
                if self.config.minimize_bit_depth {
                    color_type.to_minimal_bit_depth().into()
                } else {
                    color_type.into()
                }
            }
            ImageFormat::Jpeg => JpegColorType::try_from(color_type).unwrap_or_default().into(),
            ImageFormat::Avif => AvifColorType::try_from(color_type).unwrap_or_default().into(),
        };

        if self.config.remove_unused_transparency && output.has_alpha() && img.color().has_alpha() {
            if alpha_is_unused(img) {
                output = output.remove_alpha();
            }
        }

        Ok(output)
    }
}
