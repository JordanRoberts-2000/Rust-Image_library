use crate::{ImageError, ImageSrc};

pub trait WithSrc<T> {
    fn with_src(self, src: impl Into<ImageSrc>) -> Result<T, ImageError>;
}

impl<T, E> WithSrc<T> for Result<T, E>
where
    E: Into<ImageError>,
{
    fn with_src(self, src: impl Into<ImageSrc>) -> Result<T, ImageError> {
        self.map_err(|e| {
            let err: ImageError = e.into().with_src(src.into());
            err
        })
    }
}
