use {crate::InnerError, image::DynamicImage};

pub trait PixelFormat {
    type Channel: Clone + 'static;
    fn extract_raw_pixels(img: &DynamicImage) -> Vec<Self::Channel>;
    fn create_image_from_raw(
        pixels: Vec<Self::Channel>, width: u32, height: u32,
    ) -> Result<DynamicImage, InnerError>;
}
