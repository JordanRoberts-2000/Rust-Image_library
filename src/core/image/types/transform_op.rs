use {
    crate::{image::Decoded, CropEdge},
    image::{imageops::FilterType, DynamicImage, GenericImageView},
    std::num::NonZeroU32,
};

#[derive(Debug, Clone, PartialEq)]
pub enum TransformOp {
    Crop(u32, u32, u32, u32),
    CropAspect(f32),
    CropPixels(CropEdge, u32),
    CropRatio(CropEdge, f32),
    CropSquare,
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
    pub fn apply(&self, decoded: &mut Decoded) {
        match decoded {
            Decoded::Static(img) => {
                self.apply_transform(img);
            }
            Decoded::Animated { frames, width, height } => {
                let total = frames.len();
                for (i, frame) in frames.iter_mut().enumerate() {
                    let buf = std::mem::take(frame.buffer_mut());
                    let mut img = DynamicImage::ImageRgba8(buf);
                    self.apply_transform(&mut img);

                    let (w, h) = img.dimensions();
                    *frame.buffer_mut() = img.to_rgba8();

                    if i + 1 == total {
                        *width = w;
                        *height = h;
                    }
                }
            }
        }
    }

    fn apply_transform(&self, img: &mut DynamicImage) {
        match *self {
            TransformOp::Crop(x, y, w, h) => {
                *img = img.crop_imm(x, y, w, h);
            }
            TransformOp::CropAspect(ratio) => {
                let ratio = ratio.max(0.001).min(1000.0);
                let (w, h) = img.dimensions();
                let current = w as f32 / h as f32;

                if (current - ratio).abs() >= f32::EPSILON {
                    let (new_w, new_h) = if current > ratio {
                        let nw = ((h as f32 * ratio).round() as u32).max(1);
                        (nw, h)
                    } else {
                        let nh = ((w as f32 / ratio).round() as u32).max(1);
                        (w, nh)
                    };
                    let x0 = (w - new_w) / 2;
                    let y0 = (h - new_h) / 2;
                    *img = img.crop_imm(x0, y0, new_w, new_h);
                }
            }
            TransformOp::CropPixels(edge, pixels) => {
                let (w, h) = img.dimensions();
                let (x, y, new_w, new_h) = match edge {
                    CropEdge::Left => (pixels, 0, w.saturating_sub(pixels), h),
                    CropEdge::Right => (0, 0, w.saturating_sub(pixels), h),
                    CropEdge::Top => (0, pixels, w, h.saturating_sub(pixels)),
                    CropEdge::Bottom => (0, 0, w, h.saturating_sub(pixels)),
                    CropEdge::Horizontal => {
                        let half = pixels / 2;
                        (half, 0, w.saturating_sub(pixels), h)
                    }
                    CropEdge::Vertical => {
                        let half = pixels / 2;
                        (0, half, w, h.saturating_sub(pixels))
                    }
                    CropEdge::All => {
                        let dx = pixels.min((w.saturating_sub(1)) / 2);
                        let dy = pixels.min((h.saturating_sub(1)) / 2);
                        (dx, dy, w.saturating_sub(2 * dx), h.saturating_sub(2 * dy))
                    }
                };
                *img = img.crop_imm(x, y, new_w, new_h);
            }

            TransformOp::CropRatio(edge, ratio) => {
                let ratio = ratio.max(0.0).min(1.0);
                let (w, h) = img.dimensions();
                let pixels = match edge {
                    CropEdge::Left | CropEdge::Right | CropEdge::Horizontal => {
                        (w as f32 * ratio).round() as u32
                    }
                    CropEdge::Top | CropEdge::Bottom | CropEdge::Vertical => {
                        (h as f32 * ratio).round() as u32
                    }
                    CropEdge::All => {
                        // For All, use the smaller dimension's ratio to maintain balance
                        w.min(h) as f32 * ratio
                    }
                    .round() as u32,
                };
                TransformOp::CropPixels(edge, pixels).apply_transform(img);
            }
            TransformOp::CropSquare => {
                let (w, h) = img.dimensions();
                if w != h {
                    let side = w.min(h);
                    let x0 = (w - side) / 2;
                    let y0 = (h - side) / 2;
                    *img = img.crop_imm(x0, y0, side, side);
                }
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
                let (current_w, current_h) = img.dimensions();
                if !current_w == w.get() && !current_h == h.get() {
                    *img = img.resize(w.get(), h.get(), FilterType::Lanczos3);
                }
            }
            TransformOp::ResizeExact(w, h) => {
                let (current_w, current_h) = img.dimensions();
                if !current_w == w.get() && !current_h == h.get() {
                    *img = img.resize_exact(w.get(), h.get(), FilterType::Lanczos3);
                }
            }
            TransformOp::ResizeToFill(w, h) => {
                let (current_w, current_h) = img.dimensions();
                if !current_w == w.get() && !current_h == h.get() {
                    *img = img.resize_to_fill(w.get(), h.get(), FilterType::Lanczos3);
                }
            }
            TransformOp::MaxSize(max) => {
                if img.width() > max.get() || img.height() > max.get() {
                    *img = img.thumbnail(max.get(), max.get());
                }
            }
            TransformOp::Grayscale => {
                if !img.color().has_color() {
                    *img = img.grayscale();
                }
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
