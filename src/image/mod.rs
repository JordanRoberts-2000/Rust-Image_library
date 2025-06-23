pub mod core;
pub mod impls {
    mod constructors;
    mod encoding;
    mod internal;
    mod metadata;
    mod output;
    mod saving;
    mod serialization;
    mod source;
    mod transformations;
}
pub mod enums;
pub mod traits;

pub mod constants {
    pub const DEFAULT_IMAGE_FILE_NAME: &str = "image";
}
