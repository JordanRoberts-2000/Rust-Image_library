use image::{imageops::FilterType, DynamicImage};

use crate::ImageConfig;

use super::core::Transform;

impl ImageConfig {
    pub fn apply_transforms(&self, img: &mut DynamicImage) {
        for transform in &self.pipeline {
            match *transform {
                Transform::Crop(x, y, w, h) => {
                    *img = img.crop_imm(x, y, w, h);
                }
                Transform::Rotate(deg) => match deg {
                    90 => *img = img.rotate90(),
                    180 => *img = img.rotate180(),
                    270 => *img = img.rotate270(),
                    _ => {}
                },
                Transform::Resize(w, h) => {
                    *img = img.resize(w, h, FilterType::Lanczos3);
                }
                Transform::ResizeExact(w, h) => {
                    *img = img.resize_exact(w, h, FilterType::Lanczos3);
                }
                Transform::ResizeToFill(w, h) => {
                    *img = img.resize_to_fill(w, h, FilterType::Lanczos3);
                }
                Transform::MaxSize(max) => {
                    if img.width() > max || img.height() > max {
                        *img = img.thumbnail(max, max);
                    }
                }
                Transform::Grayscale => {
                    *img = img.grayscale();
                }
                Transform::Contrast(c) => {
                    *img = img.adjust_contrast(c);
                }
                Transform::Blur(sigma) => {
                    *img = img.blur(sigma);
                }
            }
        }
    }
}
