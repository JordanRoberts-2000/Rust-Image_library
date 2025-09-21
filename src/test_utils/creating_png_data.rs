use {
    image::{DynamicImage, ImageFormat, Rgba, RgbaImage},
    std::{io::Cursor, path::Path},
};

pub fn write_png(path: &Path) {
    let (w, h) = (2, 3);
    let img = RgbaImage::from_pixel(w, h, Rgba([0, 0, 0, 255]));
    DynamicImage::ImageRgba8(img).save(path).expect("failed to save test png");
}

pub fn png_bytes() -> Vec<u8> {
    let (w, h) = (2, 3);
    let img = RgbaImage::from_pixel(w, h, Rgba([0, 0, 0, 255]));
    let dynimg = DynamicImage::ImageRgba8(img);
    let mut buf = Vec::new();
    dynimg.write_to(&mut Cursor::new(&mut buf), ImageFormat::Png).expect("failed to encode PNG");
    buf
}
