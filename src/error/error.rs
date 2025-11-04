use {
    crate::{image::ImageOrigin, ErrorKind},
    std::fmt,
};

#[derive(Debug)]
pub struct ImageError {
    kind: ErrorKind,
    origin: Option<ImageOrigin>,
}

impl ImageError {
    pub fn new(kind: impl Into<ErrorKind>) -> Self {
        Self { kind: kind.into(), origin: None }
    }

    pub fn origin(&self) -> Option<&ImageOrigin> {
        self.origin.as_ref()
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub(crate) fn with_origin(mut self, origin: impl Into<ImageOrigin>) -> Self {
        self.origin = Some(origin.into());
        self
    }
}

impl std::error::Error for ImageError {}

impl<E> From<E> for ImageError
where
    ErrorKind: From<E>,
{
    fn from(e: E) -> Self {
        ImageError { origin: None, kind: e.into() }
    }
}

impl fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match &self.origin {
            Some(origin) => format!("Error (from {}): {}", origin, self.kind),
            None => format!("Error: {}", self.kind),
        };

        write!(f, "{}", message)
    }
}
