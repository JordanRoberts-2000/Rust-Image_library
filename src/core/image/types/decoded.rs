use {
    crate::{encoding::ColorType, ErrorKind, PixelFormat, Result},
    image::{DynamicImage, Frame, GenericImageView},
    nonempty::NonEmpty,
    std::{borrow::Cow, fmt},
};

#[derive(Clone)]
pub enum Decoded {
    Static(DynamicImage),
    Animated { frames: NonEmpty<Frame>, width: u32, height: u32 },
}

impl Decoded {
    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            Decoded::Static(img) => img.dimensions(),
            Decoded::Animated { width, height, .. } => (*width, *height),
        }
    }

    pub fn color(&self) -> Result<ColorType> {
        match self {
            Decoded::Static(img) => img.color().try_into(),
            Decoded::Animated { .. } => image::ColorType::Rgba8.try_into(),
        }
    }

    pub fn memory_bytes(&self) -> usize {
        match self {
            Decoded::Static(img) => img.as_bytes().len(),
            Decoded::Animated { frames, .. } => {
                frames.iter().map(|f| f.buffer().as_raw().len()).sum()
            }
        }
    }

    pub(crate) fn img(&self) -> Cow<'_, DynamicImage> {
        match self {
            Decoded::Static(img) => Cow::Borrowed(img),
            Decoded::Animated { frames, .. } => {
                Cow::Owned(DynamicImage::ImageRgba8(frames.first().buffer().clone()))
            }
        }
    }

    pub fn bytes(&self) -> &[u8] {
        match self {
            Decoded::Static(img) => img.as_bytes(),
            Decoded::Animated { frames, .. } => frames.first().buffer().as_raw(),
        }
    }

    pub fn as_bytes<'a, P: PixelFormat>(&'a self) -> Cow<'a, [P::Channel]> {
        P::from_decoded(self)
    }

    pub fn frames(&self) -> Result<Vec<DynamicImage>> {
        match self {
            Decoded::Static(_) => Err(ErrorKind::NotAnimated.into()),
            Decoded::Animated { frames, .. } => {
                let imgs = frames
                    .into_iter()
                    .map(|f| DynamicImage::ImageRgba8(f.buffer().clone()))
                    .collect::<Vec<_>>();
                Ok(imgs)
            }
        }
    }

    pub fn first_frame(&mut self) -> Result<&mut Self> {
        match self {
            Decoded::Static(_) => Err(ErrorKind::NotAnimated.into()),
            Decoded::Animated { frames, .. } => {
                *self = Decoded::Static(DynamicImage::ImageRgba8(frames.first().buffer().clone()));
                Ok(self)
            }
        }
    }

    pub fn last_frame(&mut self) -> Result<&mut Self> {
        match self {
            Decoded::Static(_) => Err(ErrorKind::NotAnimated.into()),
            Decoded::Animated { frames, .. } => {
                *self = Decoded::Static(DynamicImage::ImageRgba8(frames.last().buffer().clone()));
                Ok(self)
            }
        }
    }

    pub fn frame(&mut self, index: usize) -> Result<&mut Self> {
        match self {
            Decoded::Static(_) => Err(ErrorKind::NotAnimated.into()),
            Decoded::Animated { frames, .. } => {
                let fr = frames
                    .get(index)
                    .ok_or(ErrorKind::FrameOutOfBounds { index, len: frames.len() })?;
                *self = Decoded::Static(DynamicImage::ImageRgba8(fr.buffer().clone()));
                Ok(self)
            }
        }
    }
}

impl fmt::Debug for Decoded {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Decoded::Static(_) => write!(f, "Static"),
            Decoded::Animated { .. } => write!(f, "Dynamic"),
        }
    }
}
