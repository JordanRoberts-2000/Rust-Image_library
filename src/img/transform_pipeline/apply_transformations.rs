use image::DynamicImage;

use crate::TransformPipeline;

impl TransformPipeline {
    pub fn apply_transforms(&self, img: &mut DynamicImage) {
        if let Some((x, y, w, h)) = self.crop_rect {
            *img = img.crop_imm(x, y, w, h);
        }
    }
}
