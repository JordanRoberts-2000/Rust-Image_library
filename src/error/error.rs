use {
    crate::{ErrorKind, ImageSrc},
    std::fmt,
};

#[derive(Debug)]
pub struct ImageError {
    kind: ErrorKind,
    src: Option<ImageSrc>,
}

impl ImageError {
    pub fn new(kind: impl Into<ErrorKind>) -> Self {
        Self { kind: kind.into(), src: None }
    }

    pub fn src(&self) -> Option<&ImageSrc> {
        self.src.as_ref()
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub(crate) fn with_src(mut self, src: impl Into<ImageSrc>) -> Self {
        self.src = Some(src.into());
        self
    }
}

impl std::error::Error for ImageError {}

impl<E> From<E> for ImageError
where
    ErrorKind: From<E>,
{
    fn from(e: E) -> Self {
        ImageError { src: None, kind: e.into() }
    }
}

impl fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match &self.src {
            Some(src) => format!("Error (from {}): {}", src, self.kind),
            None => format!("Error: {}", self.kind),
        };

        write!(f, "{}", message)
    }
}
