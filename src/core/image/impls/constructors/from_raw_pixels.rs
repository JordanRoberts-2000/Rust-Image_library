use {
    crate::{
        image::{ImageConfig, ImageOrigin},
        Image, PixelFormat, Result, WithOrigin,
    },
    std::{borrow::Cow, cell::RefCell},
};

impl Image {
    pub fn from_raw_pixels<'a, F>(
        pixels: impl Into<Cow<'a, [F::Channel]>>, width: u32, height: u32,
    ) -> Result<Self>
    where
        F: PixelFormat,
        F::Channel: 'a,
    {
        let pixels: Vec<F::Channel> = match pixels.into() {
            Cow::Owned(v) => v,
            Cow::Borrowed(s) => s.to_vec(),
        };
        let decoded = F::into_decoded(pixels, width, height).with_origin(ImageOrigin::RawPixels)?;
        Ok(Self {
            origin: ImageOrigin::RawPixels,
            decoded: RefCell::new(decoded),
            config: ImageConfig::default(),
            format: None,
        })
    }
}

// #[cfg(test)]
// mod tests {
//     use {
//         super::*,
//         crate::pixel::{La8, Rgb16, Rgb8, Rgba16, Rgba8, L8},
//     };

//     // ------ helpers ------

//     fn seq_u8(n: usize) -> Vec<u8> {
//         (0..n).map(|i| i as u8).collect()
//     }
//     fn seq_u16(n: usize) -> Vec<u16> {
//         (0..n).map(|i| i as u16).collect()
//     }

//     fn assert_src_rawpixels(img: &Image) {
//         match img.src {
//             ImageSrc::RawPixels => {}
//             _ => panic!("expected ImageSrc::RawPixels"),
//         }
//     }

//     // ------ u8 variants ------

//     #[test]
//     fn from_raw_pixels_rgb8_ok_borrowed() -> Result<()> {
//         let (w, h, c) = (2, 2, 3);
//         let pixels = seq_u8((w * h * c) as usize); // borrowed
//         let img = Image::from_raw_pixels::<Rgb8>(&pixels, w, h)?;
//         assert_src_rawpixels(&img);
//         assert_eq!(img.width(), w);
//         assert_eq!(img.height(), h);

//         let raw = {
//             let data = img.data.borrow();
//             match &*data {
//                 ImageData::RawPixels(di) => di.to_rgb8().into_raw(),
//                 _ => panic!("expected RawPixels"),
//             }
//         };
//         assert_eq!(raw, pixels);
//         Ok(())
//     }

//     #[test]
//     fn from_raw_pixels_rgb8_ok_owned() -> Result<()> {
//         let (w, h, c) = (3, 2, 3);
//         let pixels = seq_u8((w * h * c) as usize); // owned
//         let expected = pixels.clone();
//         let img = Image::from_raw_pixels::<Rgb8>(pixels, w, h)?;
//         assert_src_rawpixels(&img);

//         let raw = {
//             let data = img.data.borrow();
//             match &*data {
//                 ImageData::RawPixels(di) => di.to_rgb8().into_raw(),
//                 _ => panic!("expected RawPixels"),
//             }
//         };
//         assert_eq!(raw, expected);
//         Ok(())
//     }

//     #[test]
//     fn from_raw_pixels_rgba8_ok() -> Result<()> {
//         let (w, h, c) = (2, 2, 4);
//         let pixels = seq_u8((w * h * c) as usize);
//         let img = Image::from_raw_pixels::<Rgba8>(&pixels, w, h)?;
//         assert_src_rawpixels(&img);

//         let raw = {
//             let data = img.data.borrow();
//             match &*data {
//                 ImageData::RawPixels(di) => di.to_rgba8().into_raw(),
//                 _ => panic!("expected RawPixels"),
//             }
//         };
//         assert_eq!(raw, pixels);
//         Ok(())
//     }

//     #[test]
//     fn from_raw_pixels_l8_ok() -> Result<()> {
//         let (w, h, c) = (3, 2, 1);
//         let pixels = seq_u8((w * h * c) as usize);
//         let img = Image::from_raw_pixels::<L8>(&pixels, w, h)?;
//         assert_src_rawpixels(&img);

//         let raw = {
//             let data = img.data.borrow();
//             match &*data {
//                 ImageData::RawPixels(di) => di.to_luma8().into_raw(),
//                 _ => panic!("expected RawPixels"),
//             }
//         };
//         assert_eq!(raw, pixels);
//         Ok(())
//     }

//     #[test]
//     fn from_raw_pixels_la8_ok() -> Result<()> {
//         let (w, h, c) = (3, 2, 2);
//         let pixels = seq_u8((w * h * c) as usize);
//         let img = Image::from_raw_pixels::<La8>(&pixels, w, h)?;
//         assert_src_rawpixels(&img);

//         let raw = {
//             let data = img.data.borrow();
//             match &*data {
//                 ImageData::RawPixels(di) => di.to_luma_alpha8().into_raw(),
//                 _ => panic!("expected RawPixels"),
//             }
//         };
//         assert_eq!(raw, pixels);
//         Ok(())
//     }

//     #[test]
//     fn from_raw_pixels_u8_mismatched_len_err() {
//         let (w, h, c) = (2, 2, 3);
//         let mut pixels = seq_u8((w * h * c) as usize);
//         pixels.pop(); // make it the wrong length
//         let res = Image::from_raw_pixels::<Rgb8>(&pixels, w, h);
//         assert!(res.is_err(), "expected error on mismatched buffer length");
//     }

//     // ------ u16 variants ------

//     #[test]
//     fn from_raw_pixels_u16_rgb16_ok() -> Result<()> {
//         let (w, h, c) = (2, 2, 3);
//         let pixels = seq_u16((w * h * c) as usize);
//         let img = Image::from_raw_pixels::<Rgb16>(&pixels, w, h)?;
//         assert_src_rawpixels(&img);

//         let raw = {
//             let data = img.data.borrow();
//             match &*data {
//                 ImageData::RawPixels(di) => di.to_rgb16().into_raw(),
//                 _ => panic!("expected RawPixels"),
//             }
//         };
//         assert_eq!(raw, pixels);
//         Ok(())
//     }

//     #[test]
//     fn from_raw_pixels_u16_rgba16_ok() -> Result<()> {
//         let (w, h, c) = (2, 2, 4);
//         let pixels = seq_u16((w * h * c) as usize);
//         let img = Image::from_raw_pixels::<Rgba16>(&pixels, w, h)?;
//         assert_src_rawpixels(&img);

//         let raw = {
//             let data = img.data.borrow();
//             match &*data {
//                 ImageData::RawPixels(di) => di.to_rgba16().into_raw(),
//                 _ => panic!("expected RawPixels"),
//             }
//         };
//         assert_eq!(raw, pixels);
//         Ok(())
//     }

//     #[test]
//     fn from_raw_pixels_u16_mismatched_len_err() {
//         let (w, h, c) = (3, 2, 4);
//         let mut pixels = seq_u16((w * h * c) as usize);
//         pixels.truncate(pixels.len() - 5); // wrong length
//         let res = Image::from_raw_pixels::<Rgba16>(&pixels, w, h);
//         assert!(res.is_err(), "expected error on mismatched buffer length");
//     }
// }
