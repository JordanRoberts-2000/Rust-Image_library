use image::{imageops::FilterType, DynamicImage};

use crate::ImageConfig;

impl ImageConfig {
    pub fn apply_transforms(&self, img: &mut DynamicImage) {
        // (order matters for efficiency)
        // Geometric transformations
        self.apply_crop(img);
        self.apply_rotation(img);
        self.apply_resize_operations(img);

        // Color/Filter adjustments
        self.apply_color_adjustments(img);
        self.apply_filters(img);
    }

    fn apply_crop(&self, img: &mut DynamicImage) {
        if let Some((x, y, w, h)) = self.crop_rect {
            *img = img.crop_imm(x, y, w, h);
        }
    }

    fn apply_rotation(&self, img: &mut DynamicImage) {
        match self.rotation_deg % 360 {
            90 => *img = img.rotate90(),
            180 => *img = img.rotate180(),
            270 => *img = img.rotate270(),
            _ => {}
        }
    }

    fn apply_resize_operations(&self, img: &mut DynamicImage) {
        // Priority order: fill -> exact -> proportional -> max_size
        // Most specific to least specific

        if let Some((w, h)) = self.resize_fill_dimensions {
            *img = img.resize_to_fill(w.max(1), h.max(1), FilterType::Lanczos3);
        }

        if let Some((w, h)) = self.resize_exact_dimensions {
            *img = img.resize_exact(w.max(1), h.max(1), FilterType::Lanczos3);
        }

        if let Some((w, h)) = self.resize_dimensions {
            *img = img.resize(w.max(1), h.max(1), FilterType::Lanczos3);
        }

        if let Some(max) = self.max_size {
            if img.width() > max || img.height() > max {
                *img = img.thumbnail(max.max(1), max.max(1));
            }
        }
    }

    fn apply_color_adjustments(&self, img: &mut DynamicImage) {
        if self.grayscale {
            *img = img.grayscale();
        }

        if let Some(c) = self.contrast {
            *img = img.adjust_contrast(c);
        }
    }

    fn apply_filters(&self, img: &mut DynamicImage) {
        if let Some(sigma) = self.blur_intensity {
            *img = img.blur(sigma);
        }
    }
}
