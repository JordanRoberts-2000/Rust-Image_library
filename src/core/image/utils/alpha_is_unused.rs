use image::DynamicImage;

pub fn alpha_is_unused(img: &DynamicImage) -> bool {
    match img {
        image::DynamicImage::ImageRgba8(buf) => {
            buf.as_raw().chunks_exact(4).all(|px| px[3] == u8::MAX)
        }
        image::DynamicImage::ImageLumaA8(buf) => {
            buf.as_raw().chunks_exact(2).all(|px| px[1] == u8::MAX)
        }
        image::DynamicImage::ImageRgba16(buf) => {
            buf.as_raw().chunks_exact(4).all(|px| px[3] == u16::MAX)
        }
        image::DynamicImage::ImageLumaA16(buf) => {
            buf.as_raw().chunks_exact(2).all(|px| px[1] == u16::MAX)
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        image::{DynamicImage, ImageBuffer, Luma, LumaA, Rgb, Rgba},
    };

    // ---------- RGBA8 ----------
    #[test]
    fn rgba8_all_opaque_is_true() {
        let img = ImageBuffer::<Rgba<u8>, _>::from_vec(
            2,
            1,
            vec![
                10, 20, 30, 255, // opaque
                40, 50, 60, 255, // opaque
            ],
        )
        .unwrap();
        let dyn_img = DynamicImage::ImageRgba8(img);
        assert!(alpha_is_unused(&dyn_img));
    }

    #[test]
    fn rgba8_with_any_transparency_is_false() {
        let img = ImageBuffer::<Rgba<u8>, _>::from_vec(
            2,
            1,
            vec![
                10, 20, 30, 255, // opaque
                40, 50, 60, 10, // not opaque
            ],
        )
        .unwrap();
        let dyn_img = DynamicImage::ImageRgba8(img);
        assert!(!alpha_is_unused(&dyn_img));
    }

    // ---------- LumaA8 ----------
    #[test]
    fn lumaa8_all_opaque_is_true() {
        let img = ImageBuffer::<LumaA<u8>, _>::from_vec(
            2,
            1,
            vec![
                128, 255, // (luma, alpha)
                200, 255,
            ],
        )
        .unwrap();
        let dyn_img = DynamicImage::ImageLumaA8(img);
        assert!(alpha_is_unused(&dyn_img));
    }

    #[test]
    fn lumaa8_with_any_transparency_is_false() {
        let img = ImageBuffer::<LumaA<u8>, _>::from_vec(
            2,
            1,
            vec![
                128, 255, 200, 0, // transparent
            ],
        )
        .unwrap();
        let dyn_img = DynamicImage::ImageLumaA8(img);
        assert!(!alpha_is_unused(&dyn_img));
    }

    // ---------- RGBA16 ----------
    #[test]
    fn rgba16_all_opaque_is_true() {
        let img = ImageBuffer::<Rgba<u16>, _>::from_vec(
            2,
            1,
            vec![
                10,
                20,
                30,
                u16::MAX, // opaque
                40,
                50,
                60,
                u16::MAX, // opaque
            ],
        )
        .unwrap();
        let dyn_img = DynamicImage::ImageRgba16(img);
        assert!(alpha_is_unused(&dyn_img));
    }

    #[test]
    fn rgba16_with_any_transparency_is_false() {
        let img = ImageBuffer::<Rgba<u16>, _>::from_vec(
            2,
            1,
            vec![
                10,
                20,
                30,
                u16::MAX,
                40,
                50,
                60,
                1024, // not opaque
            ],
        )
        .unwrap();
        let dyn_img = DynamicImage::ImageRgba16(img);
        assert!(!alpha_is_unused(&dyn_img));
    }

    // ---------- LumaA16 ----------
    #[test]
    fn lumaa16_all_opaque_is_true() {
        let img = ImageBuffer::<LumaA<u16>, _>::from_vec(2, 1, vec![128, u16::MAX, 200, u16::MAX])
            .unwrap();
        let dyn_img = DynamicImage::ImageLumaA16(img);
        assert!(alpha_is_unused(&dyn_img));
    }

    #[test]
    fn lumaa16_with_any_transparency_is_false() {
        let img = ImageBuffer::<LumaA<u16>, _>::from_vec(
            2,
            1,
            vec![
                128,
                u16::MAX,
                200,
                0, // transparent
            ],
        )
        .unwrap();
        let dyn_img = DynamicImage::ImageLumaA16(img);
        assert!(!alpha_is_unused(&dyn_img));
    }

    // ---------- Formats without alpha (should be false) ----------
    #[test]
    fn rgb8_has_no_alpha_is_false() {
        let img = ImageBuffer::<Rgb<u8>, _>::from_vec(1, 1, vec![1, 2, 3]).unwrap();
        let dyn_img = DynamicImage::ImageRgb8(img);
        assert!(!alpha_is_unused(&dyn_img));
    }

    #[test]
    fn luma8_has_no_alpha_is_false() {
        let img = ImageBuffer::<Luma<u8>, _>::from_vec(1, 1, vec![128]).unwrap();
        let dyn_img = DynamicImage::ImageLuma8(img);
        assert!(!alpha_is_unused(&dyn_img));
    }
}
