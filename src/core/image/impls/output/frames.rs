use crate::{image::Decoded, Image, Images, Result};

impl Image {
    pub fn is_animated(&self) -> bool {
        matches!(*self.decoded.borrow(), Decoded::Animated { .. })
    }

    pub fn disable_animation(&mut self, disable: bool) -> &mut Self {
        if disable {
            self.decoded.borrow_mut().first_frame();
        }
        self
    }

    pub fn frame_count(&self) -> usize {
        match &*self.decoded.borrow() {
            Decoded::Static(_) => 1,
            Decoded::Animated { frames, .. } => frames.len(),
        }
    }

    pub fn first_frame(&mut self) -> Result<&mut Self> {
        self.decoded.borrow_mut().first_frame()?;
        Ok(self)
    }

    pub fn last_frame(&mut self) -> Result<&mut Self> {
        self.decoded.borrow_mut().last_frame()?;
        Ok(self)
    }

    pub fn frame(&mut self, index: usize) -> Result<&mut Self> {
        self.decoded.borrow_mut().frame(index)?;
        Ok(self)
    }

    pub fn frames(&self) -> Result<Images> {
        let imgs = self.decoded.borrow().frames()?;

        let images: Vec<Image> = imgs
            .into_iter()
            .map(|img| {
                let mut out = self.clone();
                out.set_decoded(Decoded::Static(img));
                out
            })
            .collect();

        Ok(Images::from_vec(images))
    }
}
