use crate::{ErrorKind, ImageError, ImageSrc, InnerError, Result};

pub trait ResultCtx<T> {
    fn ctx(self, kind: ErrorKind, src: Option<&ImageSrc>) -> Result<T>;
}

impl<T, E> ResultCtx<T> for std::result::Result<T, E>
where
    E: Into<InnerError>,
{
    fn ctx(self, kind: ErrorKind, src: Option<&ImageSrc>) -> Result<T> {
        self.map_err(|e| ImageError::new(kind, src.cloned(), e))
    }
}
