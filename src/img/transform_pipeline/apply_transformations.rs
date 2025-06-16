use image::DynamicImage;

use crate::TransformPipeline;

impl TransformPipeline {
    pub fn apply_transforms(&self, img: &mut DynamicImage) {
        if let Some((x, y, w, h)) = self.crop_rect {
            *img = img.crop_imm(x, y, w, h);
        }

        match self.rotation_deg % 360 {
            90 => *img = img.rotate90(),
            180 => *img = img.rotate180(),
            270 => *img = img.rotate270(),
            _ => {}
        }
    }
}
