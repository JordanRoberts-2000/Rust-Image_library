mod core;
mod impls;
mod traits;
mod types;
mod utils;

pub(crate) use {core::ImageEntry, utils::*};
pub use {
    core::Images,
    traits::Archive,
    types::{FormatFilter, FromDirConfig, ImagesBuilder},
};

pub mod archive_formats {
    pub use super::types::{Tar, TarGz, Zip};
}
