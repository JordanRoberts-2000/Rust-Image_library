mod constants;
mod image_data {
    pub mod corrupted;
    pub mod encoded;
    pub mod file;
    pub mod raw;
}
pub mod server {
    mod register_image;
    mod register_not_found;
    pub use {
        register_image::{register_corrupted_header_image, register_image},
        register_not_found::register_not_found,
    };
}

pub use {
    constants::MOCK_IMAGE_DIMENSIONS,
    image_data::{corrupted::*, encoded::*, file::*, raw::*},
};
