mod repos {
    pub mod fs;
    pub mod io;
}
mod services {
    pub mod image;
    pub mod metadata;
}
mod http_client;

#[cfg(test)]
pub use services::image::MockImageService;
pub use {
    http_client::HttpClient,
    repos::{fs::FsRepo, io::IoRepo},
    services::{image::ImageService, metadata::MetadataService},
};
