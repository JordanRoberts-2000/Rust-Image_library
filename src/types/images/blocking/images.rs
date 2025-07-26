use {
    crate::{blocking::Image, images::types::ImagesConfig, Result, ValidationError},
    std::ops::{Index, IndexMut, RangeBounds},
};

const DEFAULT_CAPACITY: usize = 8;

#[derive(Debug, Clone, PartialEq)]
pub struct Images {
    pub(crate) inner: Vec<Image>,
    pub(crate) config: ImagesConfig,
}

impl Index<usize> for Images {
    type Output = Image;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl IndexMut<usize> for Images {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

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

impl Images {
    pub fn new() -> Self {
        Self { inner: Vec::with_capacity(DEFAULT_CAPACITY), config: ImagesConfig::default() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self { inner: Vec::with_capacity(capacity), config: ImagesConfig::default() }
    }

    pub fn from_vec(images: Vec<Image>) -> Self {
        Self { inner: images, config: ImagesConfig::default() }
    }

    pub fn push(&mut self, image: Image) {
        self.inner.push(image);
    }

    pub fn extend<I: IntoIterator<Item = Image>>(&mut self, iter: I) {
        self.inner.extend(iter);
    }

    pub fn append(&mut self, other: &mut Images) {
        self.inner.append(&mut other.inner);
    }

    pub fn insert(&mut self, index: usize, image: Image) -> Result<()> {
        if index <= self.inner.len() {
            self.inner.insert(index, image);
            Ok(())
        } else {
            Err(ValidationError::IndexOutOfBounds(index).into())
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Image> {
        self.inner.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Image> {
        self.inner.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&Image> {
        self.inner.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Image> {
        self.inner.get_mut(index)
    }

    pub fn remove(&mut self, index: usize) -> Option<Image> {
        if index < self.inner.len() {
            Some(self.inner.remove(index))
        } else {
            None
        }
    }

    pub fn pop(&mut self) -> Option<Image> {
        self.inner.pop()
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&Image) -> bool,
    {
        self.inner.retain(f);
    }

    pub fn drain<R>(&mut self, range: R) -> std::vec::Drain<'_, Image>
    where
        R: RangeBounds<usize>,
    {
        self.inner.drain(range)
    }
}
