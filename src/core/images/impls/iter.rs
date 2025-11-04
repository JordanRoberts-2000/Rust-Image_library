use crate::{images::ImageEntry, Image, Images, Result};

pub struct ImagesIntoIter {
    imgs: std::vec::IntoIter<Image>,
    entries: std::vec::IntoIter<ImageEntry>,
}

impl Iterator for ImagesIntoIter {
    type Item = Result<Image>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(img) = self.imgs.next() {
            return Some(Ok(img));
        }

        self.entries.next().map(|entry| {
            let mut img = Image::try_from(entry.src)?;
            img.config = entry.config;
            Ok(img)
        })
    }
}

impl IntoIterator for Images {
    type Item = Result<Image>;
    type IntoIter = ImagesIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        ImagesIntoIter { imgs: self.image_vec.into_iter(), entries: self.entry_vec.into_iter() }
    }
}
