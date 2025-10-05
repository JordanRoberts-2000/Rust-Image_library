use {crate::Image, std::num::NonZeroU32};

impl Image {
    pub(crate) fn set_width(&mut self, width: NonZeroU32) {
        self.metadata.width = width;
    }

    pub(crate) fn set_height(&mut self, height: NonZeroU32) {
        self.metadata.height = height;
    }

    pub(crate) fn set_size(&mut self, width: NonZeroU32, height: NonZeroU32) {
        self.set_width(width);
        self.set_height(height);
    }
}
