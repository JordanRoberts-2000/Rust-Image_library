mod format_filter;
mod from_folder_config;
mod images_builder;

pub use {
    format_filter::FormatFilter, from_folder_config::FromFolderConfig,
    images_builder::ImagesBuilder,
};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum CollisionStrategy {
    #[default]
    Dedupe,
    RemoveDuplicates,
    Error,
}
