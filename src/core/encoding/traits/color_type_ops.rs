use crate::encoding::BitDepth;

pub trait AlphaChannelOps {
    fn has_alpha(&self) -> bool;
    fn remove_alpha(self) -> Self;
    fn ensure_alpha(self) -> Self;
}

pub trait ColorTypeOps {
    fn channels(&self) -> u8;
    fn bit_depth(&self) -> BitDepth;
    fn supports_grayscale() -> bool;
    fn supports_transparency() -> bool;
}

pub trait GrayscaleOps {
    fn is_grayscale(&self) -> bool;
    fn to_grayscale(self) -> Self;
    fn to_color(self) -> Self;
}
