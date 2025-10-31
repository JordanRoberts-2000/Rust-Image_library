use {
    crate::{image::Decoded, Image, ImageSrc},
    fs_ext::{CollisionStrategy, ParentPolicy, RenameOptions, WriteOptions},
    std::cell::Ref,
};

impl Image {
    pub(crate) fn src(&self) -> ImageSrc {
        self.src.clone()
    }

    pub(crate) fn decoded(&self) -> Ref<Decoded> {
        let mut decoded = self.decoded.borrow_mut();

        let mut ops = self.config.pipeline.borrow_mut();
        if !ops.is_empty() {
            for op in ops.drain(..) {
                op.apply(&mut decoded);
            }
        }
        drop(decoded);

        self.decoded.borrow()
    }

    pub(crate) fn write_options(&self) -> WriteOptions {
        WriteOptions {
            parent: ParentPolicy::RequireExists,
            collision: self
                .config
                .collision_strategy
                .clone()
                .unwrap_or(CollisionStrategy::Rename(RenameOptions::Counter)),
        }
    }
}
