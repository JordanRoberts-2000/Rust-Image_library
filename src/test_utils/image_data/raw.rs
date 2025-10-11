use crate::{
    encoding::{ColorType, ColorTypeOps},
    test_utils::MOCK_IMAGE_DIMENSIONS,
};

pub fn raw_pixel_data(color_type: ColorType) -> Vec<u8> {
    let (w, h) = MOCK_IMAGE_DIMENSIONS;

    let size = color_type.buffer_size(w, h);
    (0..size).map(|i| (i % 256) as u8).collect()
}
