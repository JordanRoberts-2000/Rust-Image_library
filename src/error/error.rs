use {
    crate::{ErrorKind, ImageSrc, InnerError},
    std::fmt,
};

#[derive(Debug)]
pub struct ImageError {
    kind: ErrorKind,
    src: Option<ImageSrc>,
    error: Box<InnerError>,
}

impl ImageError {
    pub fn new(kind: ErrorKind, src: Option<ImageSrc>, error: impl Into<InnerError>) -> Self {
        Self { kind, src, error: Box::new(error.into()) }
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
    pub fn src(&self) -> Option<&ImageSrc> {
        self.src.as_ref()
    }
    pub fn inner(&self) -> &InnerError {
        &self.error
    }
}

impl fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let context = match &self.src {
            Some(src) => format!("Error {}, from {}: ", self.kind, src),
            None => format!("Error {}: ", self.kind),
        };

        write!(f, "{}{}", context, self.error)
    }
}
