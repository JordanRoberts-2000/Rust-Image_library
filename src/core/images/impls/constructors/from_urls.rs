use crate::{Image, Images, Result};

impl Images {
    pub fn from_urls<I, U>(urls: I) -> Result<Self>
    where
        I: IntoIterator<Item = U>,
        U: AsRef<str>,
    {
        let mut images = Vec::new();

        for url in urls {
            let image = Image::from_url(url)?;
            images.push(image);
        }

        Ok(Self::from_vec(images))
    }
}
