use crate::{BitDepth, ColorType};

pub fn create_rgb8_data(width: u32, height: u32) -> Vec<u8> {
    create_image_data(width, height, ColorType::Rgb8)
}

pub fn create_image_data(width: u32, height: u32, color_type: ColorType) -> Vec<u8> {
    let channels = color_type.channels();
    let bytes_per_channel = match color_type.bit_depth() {
        BitDepth::Eight => 1,
        BitDepth::Sixteen => 2,
    };

    let size = (width * height * channels as u32 * bytes_per_channel as u32) as usize;
    (0..size).map(|i| (i % 256) as u8).collect()
}
