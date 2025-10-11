pub trait AlphaChannelOps {
    fn has_alpha(&self) -> bool;
    fn remove_alpha(self) -> Self;
    fn ensure_alpha(self) -> Self;
}

pub trait ColorTypeOps {
    fn channels(&self) -> u8;
    fn bit_depth(&self) -> u8;
    fn bytes_per_pixel(&self) -> usize {
        (self.channels() as usize) * (self.bit_depth() as usize / 8)
    }
    fn buffer_size(&self, w: u32, h: u32) -> u64 {
        (w as u64) * (h as u64) * (self.bytes_per_pixel() as u64)
    }
    fn supports_grayscale() -> bool;
    fn supports_transparency() -> bool;
}

pub trait GrayscaleOps {
    fn is_grayscale(&self) -> bool;
    fn to_grayscale(self) -> Self;
    fn to_color(self) -> Self;
}
