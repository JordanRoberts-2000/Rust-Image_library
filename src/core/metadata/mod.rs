mod core;
mod impls {
    mod constructors {
        mod from_bytes;
        mod from_path;
        mod from_reader;
        mod from_url;
    }
    mod internal;
    mod output;
}

pub use core::ImageMetadata;
