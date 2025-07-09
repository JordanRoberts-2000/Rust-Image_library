mod image;
pub mod traits;
mod repositories {
    pub mod fs;
}
mod impls;
pub mod dependencies {
    pub use {
        image::{ImageDeps, MockImageDeps},
        metadata::Metadata,
        url_downloader::UrlDownloader,
    };

    mod image;
    mod metadata;
    mod url_downloader;
}

pub use {image::Image, repositories::fs::FsRepo};
