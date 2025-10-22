use image::codecs::gif as img_gif;

#[derive(Clone, Copy, Debug)]
pub enum GifRepeat {
    Finite(u16),
    Infinite,
}

impl From<GifRepeat> for img_gif::Repeat {
    fn from(r: GifRepeat) -> Self {
        match r {
            GifRepeat::Infinite => img_gif::Repeat::Infinite,
            GifRepeat::Finite(n) => img_gif::Repeat::Finite(n),
        }
    }
}
