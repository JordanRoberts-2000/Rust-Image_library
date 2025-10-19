use crate::{image::TransformOp, CropEdge, Image};

impl Image {
    pub fn crop_aspect(&mut self, ratio: f32) -> &mut Self {
        self.config.pipeline.borrow_mut().push(TransformOp::CropAspect(ratio));
        self
    }

    pub fn crop_pixels(&mut self, edge: CropEdge, pixels: u32) -> &mut Self {
        self.config.pipeline.borrow_mut().push(TransformOp::CropPixels(edge, pixels));
        self
    }

    pub fn crop_ratio(&mut self, edge: CropEdge, ratio: f32) -> &mut Self {
        self.config.pipeline.borrow_mut().push(TransformOp::CropRatio(edge, ratio));
        self
    }

    pub fn crop_square(&mut self) -> &mut Self {
        self.config.pipeline.borrow_mut().push(TransformOp::CropSquare);
        self
    }

    pub fn crop(&mut self, x: u32, y: u32, w: u32, h: u32) -> &mut Self {
        self.config.pipeline.borrow_mut().push(TransformOp::Crop(x, y, w, h));
        self
    }

    pub fn inset(&mut self, pixels: u32) -> &mut Self {
        self.crop_pixels(CropEdge::All, pixels)
    }

    pub fn inset_ratio(&mut self, ratio: f32) -> &mut Self {
        self.crop_ratio(CropEdge::All, ratio)
    }
}
