use image::imageops::FilterType;

use crate::{Image, Result, TransformOp};

impl Image {
    pub fn apply_transforms(&mut self) -> Result<()> {
        if self.config.pipeline.is_empty() {
            return Ok(());
        }
        // Take pipeline before mutable borrow, reset to empty vec
        let pipeline = std::mem::take(&mut self.config.pipeline);

        let img = self.get_decoded()?;

        for transform in pipeline {
            match transform {
                TransformOp::Crop(x, y, w, h) => {
                    *img = img.crop_imm(x, y, w, h);
                }
                TransformOp::Rotate(deg) => match deg {
                    90 => *img = img.rotate90(),
                    180 => *img = img.rotate180(),
                    270 => *img = img.rotate270(),
                    _ => {}
                },
                TransformOp::Resize(w, h) => {
                    *img = img.resize(w, h, FilterType::Lanczos3);
                }
                TransformOp::ResizeExact(w, h) => {
                    *img = img.resize_exact(w, h, FilterType::Lanczos3);
                }
                TransformOp::ResizeToFill(w, h) => {
                    *img = img.resize_to_fill(w, h, FilterType::Lanczos3);
                }
                TransformOp::MaxSize(max) => {
                    if img.width() > max || img.height() > max {
                        *img = img.thumbnail(max, max);
                    }
                }
                TransformOp::Grayscale => {
                    *img = img.grayscale();
                }
                TransformOp::Contrast(c) => {
                    *img = img.adjust_contrast(c);
                }
                TransformOp::Blur(sigma) => {
                    *img = img.blur(sigma);
                }
            }
        }

        Ok(())
    }
}
