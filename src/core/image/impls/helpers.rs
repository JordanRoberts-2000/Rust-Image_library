use crate::{
    encoding::{ColorTypeOps, EncodeColorTypeOps},
    image::{utils::alpha_is_unused, Decoded},
    Image, Result,
};

impl Image {
    pub(crate) fn resolve_color_type<F: EncodeColorTypeOps + ColorTypeOps>(
        &self, decoded: &Decoded,
    ) -> Result<F> {
        let ct = self.color_type()?;

        let mut format_ct = F::from_color_type_lossy(ct);

        if self.config.minimize_bit_depth {
            format_ct = format_ct.to_minimal_bit_depth();
        }

        if self.config.remove_unused_transparency
            && format_ct.has_alpha()
            && alpha_is_unused(&decoded.img())
        {
            format_ct = format_ct.remove_alpha();
        }

        Ok(format_ct)
    }
}
