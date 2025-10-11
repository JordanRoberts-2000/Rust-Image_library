macro_rules! forward_grayscale_impls {
    () => {
        #[inline]
        pub fn is_grayscale(&self) -> bool {
            <Self as crate::encoding::GrayscaleOps>::is_grayscale(self)
        }
        #[inline]
        pub fn to_grayscale(self) -> Self {
            <Self as crate::encoding::GrayscaleOps>::to_grayscale(self)
        }
        #[inline]
        pub fn to_color(self) -> Self {
            <Self as crate::encoding::GrayscaleOps>::to_color(self)
        }
    };
}

macro_rules! forward_color_type_impls {
    () => {
        #[inline]
        pub fn channels(&self) -> u8 {
            <Self as $crate::encoding::ColorTypeOps>::channels(self)
        }
        #[inline]
        pub fn bit_depth(&self) -> u8 {
            <Self as $crate::encoding::ColorTypeOps>::bit_depth(self)
        }
        #[inline]
        pub fn bytes_per_pixel(&self) -> usize {
            <Self as $crate::encoding::ColorTypeOps>::bytes_per_pixel(self)
        }
        #[inline]
        pub fn buffer_size(&self, w: u32, h: u32) -> u64 {
            <Self as $crate::encoding::ColorTypeOps>::buffer_size(self, w, h)
        }
        #[inline]
        pub fn supports_grayscale() -> bool {
            <Self as $crate::encoding::ColorTypeOps>::supports_grayscale()
        }
        #[inline]
        pub fn supports_transparency() -> bool {
            <Self as $crate::encoding::ColorTypeOps>::supports_transparency()
        }
    };
}

macro_rules! forward_transparency_impls {
    () => {
        #[inline]
        pub fn has_alpha(&self) -> bool {
            <Self as $crate::encoding::AlphaChannelOps>::has_alpha(self)
        }
        #[inline]
        pub fn remove_alpha(self) -> Self {
            <Self as $crate::encoding::AlphaChannelOps>::remove_alpha(self)
        }
        #[inline]
        pub fn ensure_alpha(self) -> Self {
            <Self as $crate::encoding::AlphaChannelOps>::ensure_alpha(self)
        }
    };
}

pub(crate) use {forward_color_type_impls, forward_grayscale_impls, forward_transparency_impls};
