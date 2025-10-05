use crate::{test_utils::MOCK_IMAGE_DIMENSIONS, BitDepth, ColorType};

pub fn raw_pixel_data(color_type: ColorType) -> Vec<u8> {
    let (w, h) = MOCK_IMAGE_DIMENSIONS;

    let channels = color_type.channels();
    let bytes_per_channel = match color_type.bit_depth() {
        BitDepth::Eight => 1,
        BitDepth::Sixteen => 2,
    };

    let size = (w * h * channels as u32 * bytes_per_channel as u32) as usize;
    (0..size).map(|i| (i % 256) as u8).collect()
}
