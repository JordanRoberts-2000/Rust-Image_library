use crate::{ImageError, ImageSrc};

pub trait WithSrc<T> {
    fn with_src(self, src: Option<&ImageSrc>) -> Result<T, ImageError>;
}

impl<T, E> WithSrc<T> for Result<T, E>
where
    E: Into<ImageError>,
{
    fn with_src(self, src: Option<&ImageSrc>) -> Result<T, ImageError> {
        self.map_err(|e| {
            let err: ImageError = e.into();
            match src {
                Some(s) => err.with_src(s.clone()),
                None => err,
            }
        })
    }
}
