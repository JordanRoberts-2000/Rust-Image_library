use {
    crate::{image::Decoded, Image},
    std::cell::RefCell,
};

impl Image {
    pub(crate) fn set_decoded(&mut self, decoded: Decoded) -> &mut Self {
        self.decoded = RefCell::new(decoded);
        self
    }
}
