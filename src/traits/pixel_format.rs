use {
    crate::{image::Decoded, Result},
    std::borrow::Cow,
};

pub trait PixelFormat {
    type Channel: Clone;

    fn from_decoded<'a>(decoded: &'a Decoded) -> Cow<'a, [Self::Channel]>;
    fn into_decoded(pixels: Vec<Self::Channel>, width: u32, height: u32) -> Result<Decoded>;
}
