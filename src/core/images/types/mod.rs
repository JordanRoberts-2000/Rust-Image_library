mod format_filter;
mod from_folder_config;
mod images_builder;
mod archive_formats {
    pub mod tar;
    pub mod tar_gz;
    pub mod zip;
}

pub use {
    archive_formats::{tar::Tar, tar_gz::TarGz, zip::Zip},
    format_filter::FormatFilter,
    from_folder_config::FromFolderConfig,
    images_builder::ImagesBuilder,
};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum CollisionStrategy {
    #[default]
    Dedupe,
    RemoveDuplicates,
    Error,
}
