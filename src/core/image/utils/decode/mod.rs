mod from_reader;
mod formats {
    pub mod gif;
    pub mod svg;
}

pub use {
    formats::{gif::decode_gif, svg::rasterize_svg},
    from_reader::from_reader,
};
