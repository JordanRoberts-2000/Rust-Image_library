use crate::{Image, Images};

impl IntoIterator for Images {
    type Item = Image;
    type IntoIter = std::vec::IntoIter<Image>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a> IntoIterator for &'a Images {
    type Item = &'a Image;
    type IntoIter = std::slice::Iter<'a, Image>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<'a> IntoIterator for &'a mut Images {
    type Item = &'a mut Image;
    type IntoIter = std::slice::IterMut<'a, Image>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter_mut()
    }
}
