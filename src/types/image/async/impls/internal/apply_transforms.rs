use image::imageops::FilterType;

use crate::{
    image::{enums::TransformOp, r#async::ImageData},
    Image, InternalError, Result,
};

impl Image {
    pub(crate) async fn apply_transforms(&self) -> Result<()> {
        let pipeline = {
            let state = self.state.read().await;
            if state.config.pipeline.is_empty() {
                return Ok(());
            }
            state.config.pipeline.clone()
        };

        self.decode().await?;

        let mut state = self.state.write().await;
        let img = match &mut state.data {
            ImageData::Decoded(ref mut img) => img,
            _ => return Err(InternalError::DecodingInvariantViolatedAfterDecodeAssignment.into()),
        };

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
                    *img = img.resize(w.get(), h.get(), FilterType::Lanczos3);
                }
                TransformOp::ResizeExact(w, h) => {
                    *img = img.resize_exact(w.get(), h.get(), FilterType::Lanczos3);
                }
                TransformOp::ResizeToFill(w, h) => {
                    *img = img.resize_to_fill(w.get(), h.get(), FilterType::Lanczos3);
                }
                TransformOp::MaxSize(max) => {
                    if img.width() > max.get() || img.height() > max.get() {
                        *img = img.thumbnail(max.get(), max.get());
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
