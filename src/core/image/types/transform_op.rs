use {
    image::{imageops::FilterType, DynamicImage},
    std::num::NonZeroU32,
};

#[derive(Debug, Clone, PartialEq)]
pub enum TransformOp {
    Crop(u32, u32, u32, u32),
    Rotate90,
    Rotate180,
    Rotate270,
    Resize(NonZeroU32, NonZeroU32),
    ResizeExact(NonZeroU32, NonZeroU32),
    ResizeToFill(NonZeroU32, NonZeroU32),
    MaxSize(NonZeroU32),
    Grayscale,
    Contrast(f32),
    Blur(f32),
}

impl TransformOp {
    pub fn apply(self, img: &mut DynamicImage) {
        match self {
            TransformOp::Crop(x, y, w, h) => {
                *img = img.crop_imm(x, y, w, h);
            }
            TransformOp::Rotate90 => {
                *img = img.rotate90();
            }
            TransformOp::Rotate180 => {
                *img = img.rotate180();
            }
            TransformOp::Rotate270 => {
                *img = img.rotate270();
            }
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
}
