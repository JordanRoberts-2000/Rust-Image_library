use crate::{image::ImageOrigin, ImageError};

pub trait WithOrigin<T> {
    fn with_origin(self, src: impl Into<ImageOrigin>) -> Result<T, ImageError>;
}

impl<T, E> WithOrigin<T> for Result<T, E>
where
    E: Into<ImageError>,
{
    fn with_origin(self, src: impl Into<ImageOrigin>) -> Result<T, ImageError> {
        self.map_err(|e| {
            let err: ImageError = e.into().with_origin(src.into());
            err
        })
    }
}
