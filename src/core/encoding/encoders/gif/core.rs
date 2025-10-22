use {
    super::{GifRepeat, GifSpeed},
    crate::{
        constants::DEFAULT_GIF_SPEED,
        encoding::{EncodingError, EncodingErrorKind, EncodingValidationError},
        ImageFormat,
    },
    image::{codecs::gif::GifEncoder as Encoder, Frame},
    std::io::Write,
};

#[derive(Debug, Clone)]
pub struct GifEncoder {
    pub(crate) speed: GifSpeed,
    pub(crate) repeat: GifRepeat,
    pub(crate) animated: bool,
}

impl GifEncoder {
    pub fn new() -> Self {
        Self { speed: DEFAULT_GIF_SPEED.into(), repeat: GifRepeat::Infinite, animated: true }
    }

    pub fn still_image() -> Self {
        Self { speed: DEFAULT_GIF_SPEED.into(), repeat: GifRepeat::Infinite, animated: false }
    }

    pub fn set_speed(mut self, speed: impl Into<GifSpeed>) -> Self {
        self.speed = speed.into();
        self
    }

    pub fn set_repeat(mut self, repeat: GifRepeat) -> Self {
        self.repeat = repeat;
        self
    }

    pub fn set_animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    pub fn encode<W, I>(&self, writer: W, frames: I) -> Result<(), EncodingError>
    where
        W: Write,
        I: IntoIterator<Item = Frame>,
    {
        let mut frames = frames.into_iter();

        let mut encoder = Encoder::new_with_speed(writer, self.speed.into());
        encoder.set_repeat(self.repeat.into()).unwrap();

        match frames.next() {
            Some(frame) => encoder.encode_frame(frame).unwrap(),
            None => {
                return Err(EncodingError::new(
                    ImageFormat::Gif,
                    EncodingErrorKind::Validation(EncodingValidationError::EmptyFrames),
                ));
            }
        };

        if self.animated {
            for frame in frames {
                encoder.encode_frame(frame).unwrap();
            }
        }

        Ok(())
    }
}

impl Default for GifEncoder {
    fn default() -> Self {
        Self::new()
    }
}
