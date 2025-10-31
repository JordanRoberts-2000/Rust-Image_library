use {
    crate::{image::Decoded, Image},
    fs_ext::CollisionStrategy,
    std::cell::RefCell,
};

impl Image {
    pub(crate) fn set_decoded(&mut self, decoded: Decoded) -> &mut Self {
        self.decoded = RefCell::new(decoded);
        self
    }

    pub fn collision_strategy(&mut self, collision_strategy: CollisionStrategy) -> &mut Self {
        self.config.collision_strategy = Some(collision_strategy);
        self
    }
}
