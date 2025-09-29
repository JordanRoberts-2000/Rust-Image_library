use {
    crate::{Image, Images},
    std::ops::{Deref, DerefMut, Index, IndexMut},
};

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

impl Deref for Images {
    type Target = Vec<Image>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for Images {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
