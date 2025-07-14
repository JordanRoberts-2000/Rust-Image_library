use image::RgbaImage;
use ravif::RGBA8;

pub fn to_rgba8_vec(image: &RgbaImage) -> Vec<RGBA8> {
    image
        .pixels()
        .map(|p| RGBA8 {
            r: p[0],
            g: p[1],
            b: p[2],
            a: p[3],
        })
        .collect()
}
