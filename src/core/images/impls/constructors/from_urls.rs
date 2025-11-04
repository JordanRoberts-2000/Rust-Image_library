use crate::{ImageSrc, Images, Result};

impl Images {
    pub fn from_urls<I, U>(urls: I) -> Result<Self>
    where
        I: IntoIterator<Item = U>,
        U: AsRef<str>,
    {
        let mut src_vec = Vec::new();

        for url in urls {
            src_vec.push(ImageSrc::Url(url.as_ref().to_string()));
        }

        Ok(Self::from_src_vec(src_vec))
    }
}
