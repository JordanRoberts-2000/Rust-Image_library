use {
    crate::{ErrorKind, Result},
    image::{DynamicImage, Frame, GenericImageView},
    std::{borrow::Cow, fmt},
};

#[derive(Clone)]
pub enum Decoded {
    Static(DynamicImage),
    Animated { frames: Vec<Frame>, width: u32, height: u32 },
}

impl Decoded {
    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            Decoded::Static(img) => img.dimensions(),
            Decoded::Animated { width, height, .. } => (*width, *height),
        }
    }

    pub fn color(&self) -> image::ColorType {
        match self {
            Decoded::Static(img) => img.color(),
            Decoded::Animated { .. } => image::ColorType::Rgba8,
        }
    }

    pub fn get_static(&self) -> Result<Cow<'_, DynamicImage>> {
        match self {
            Decoded::Static(img) => Ok(Cow::Borrowed(img)),
            Decoded::Animated { frames, .. } => {
                let first = frames.first().ok_or(ErrorKind::EmptyGif)?;
                let di = DynamicImage::ImageRgba8(first.buffer().clone());
                Ok(Cow::Owned(di))
            }
        }
    }

    pub fn into_static(&mut self) -> Result<&mut Self> {
        match self {
            Decoded::Static(_) => Ok(self),
            Decoded::Animated { frames, .. } => {
                let mut frames_taken = std::mem::take(frames);
                let first = frames_taken.drain(..).next().ok_or(ErrorKind::EmptyGif)?;

                let img = DynamicImage::ImageRgba8(first.into_buffer());
                *self = Decoded::Static(img);

                Ok(self)
            }
        }
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
                let fr = frames.first().ok_or(ErrorKind::EmptyGif)?;
                *self = Decoded::Static(DynamicImage::ImageRgba8(fr.buffer().clone()));
                Ok(self)
            }
        }
    }

    pub fn last_frame(&mut self) -> Result<&mut Self> {
        match self {
            Decoded::Static(_) => Err(ErrorKind::NotAnimated.into()),
            Decoded::Animated { frames, .. } => {
                let fr = frames.last().ok_or(ErrorKind::EmptyGif)?;
                *self = Decoded::Static(DynamicImage::ImageRgba8(fr.buffer().clone()));
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
