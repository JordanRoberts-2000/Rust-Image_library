use {
    crate::Image,
    std::io::{self, Seek, Write},
};

pub trait Archive {
    fn write_images(writer: impl Write + Seek, images: &mut [Image]) -> io::Result<()>;
}
