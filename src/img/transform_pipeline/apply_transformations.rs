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

        if self.grayscale {
            *img = img.grayscale();
        }

        if let Some(c) = self.contrast {
            *img = img.adjust_contrast(c);
        }

        if let Some(sigma) = self.blur_intensity {
            *img = img.blur(sigma);
        }
    }
}
