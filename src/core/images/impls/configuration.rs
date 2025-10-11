use crate::{
    encoding::{AvifConfig, JpegConfig, WebpConfig},
    Blur, CropEdge, ImageFormat, Images,
};

impl Images {
    pub fn configure_jpeg(&mut self, config: JpegConfig) -> &mut Self {
        for image in &mut self.inner {
            image.configure_jpeg(config.clone());
        }
        self
    }

    pub fn configure_avif(&mut self, config: AvifConfig) -> &mut Self {
        for image in &mut self.inner {
            image.configure_avif(config.clone());
        }
        self
    }

    pub fn configure_webp(&mut self, config: WebpConfig) -> &mut Self {
        for image in &mut self.inner {
            image.configure_webp(config.clone());
        }
        self
    }

    pub fn lossless(&mut self) -> &mut Self {
        for image in &mut self.inner {
            image.lossless();
        }
        self
    }

    pub fn jpeg(&mut self) -> &mut Self {
        for image in &mut self.inner {
            image.jpeg();
        }
        self
    }

    pub fn png(&mut self) -> &mut Self {
        for image in &mut self.inner {
            image.png();
        }
        self
    }

    pub fn webp(&mut self) -> &mut Self {
        for image in &mut self.inner {
            image.webp();
        }
        self
    }

    pub fn avif(&mut self) -> &mut Self {
        for image in &mut self.inner {
            image.avif();
        }
        self
    }

    pub fn to_format(&mut self, format: ImageFormat) -> &mut Self {
        for image in &mut self.inner {
            image.to_format(format);
        }
        self
    }

    pub fn quality(&mut self, quality: u8) -> &mut Self {
        for image in &mut self.inner {
            image.quality(quality);
        }
        self
    }

    pub fn minimize_bit_depth(&mut self) -> &mut Self {
        for image in &mut self.inner {
            image.minimize_bit_depth();
        }
        self
    }

    pub fn remove_unused_transparency(&mut self) -> &mut Self {
        for image in &mut self.inner {
            image.remove_unused_transparency();
        }
        self
    }

    pub fn prefix(&mut self, prefix: impl AsRef<str>) -> &mut Self {
        for image in &mut self.inner {
            image.prefix(prefix.as_ref());
        }
        self
    }

    pub fn suffix(&mut self, suffix: impl AsRef<str>) -> &mut Self {
        for image in &mut self.inner {
            image.suffix(suffix.as_ref());
        }
        self
    }

    pub fn max_size(&mut self, size: u32) -> &mut Self {
        for image in &mut self.inner {
            image.max_size(size);
        }
        self
    }

    pub fn resize(&mut self, width: u32, height: u32) -> &mut Self {
        for image in &mut self.inner {
            image.resize(width, height);
        }
        self
    }

    pub fn resize_exact(&mut self, width: u32, height: u32) -> &mut Self {
        for image in &mut self.inner {
            image.resize_exact(width, height);
        }
        self
    }

    pub fn resize_fill(&mut self, width: u32, height: u32) -> &mut Self {
        for image in &mut self.inner {
            image.resize_fill(width, height);
        }
        self
    }

    pub fn adjust_contrast(&mut self, contrast: i32) -> &mut Self {
        for image in &mut self.inner {
            image.adjust_contrast(contrast);
        }
        self
    }

    pub fn blur(&mut self, intensity: Blur) -> &mut Self {
        for image in &mut self.inner {
            image.blur(intensity);
        }
        self
    }

    pub fn grayscale(&mut self) -> &mut Self {
        for image in &mut self.inner {
            image.grayscale();
        }
        self
    }

    pub fn rotate_90(&mut self) -> &mut Self {
        for image in &mut self.inner {
            image.rotate_90();
        }
        self
    }

    pub fn rotate_180(&mut self) -> &mut Self {
        for image in &mut self.inner {
            image.rotate_180();
        }
        self
    }

    pub fn rotate_270(&mut self) -> &mut Self {
        for image in &mut self.inner {
            image.rotate_270();
        }
        self
    }

    pub fn crop_aspect(&mut self, ratio: f32) -> &mut Self {
        for image in &mut self.inner {
            image.crop_aspect(ratio);
        }
        self
    }

    pub fn crop_pixels(&mut self, edge: CropEdge, pixels: u32) -> &mut Self {
        for image in &mut self.inner {
            image.crop_pixels(edge, pixels);
        }
        self
    }

    pub fn crop_ratio(&mut self, edge: CropEdge, ratio: f32) -> &mut Self {
        for image in &mut self.inner {
            image.crop_ratio(edge, ratio);
        }
        self
    }

    pub fn crop_square(&mut self) -> &mut Self {
        for image in &mut self.inner {
            image.crop_square();
        }
        self
    }

    pub fn crop(&mut self, x: u32, y: u32, w: u32, h: u32) -> &mut Self {
        for image in &mut self.inner {
            image.crop(x, y, w, h);
        }
        self
    }

    pub fn inset_ratio(&mut self, ratio: f32) -> &mut Self {
        for image in &mut self.inner {
            image.inset_ratio(ratio);
        }
        self
    }

    pub fn inset(&mut self, pixels: u32) -> &mut Self {
        for image in &mut self.inner {
            image.inset(pixels);
        }
        self
    }
}
