// //jpeg:
// use image::{codecs::jpeg::JpegEncoder, load_from_memory, ExtendedColorType};
// let mut encoder = JpegEncoder::new_with_quality(&mut buffer, quality);

// encoder
//     .encode(
//         self.img.to_rgb8().as_raw(),
//         self.width,
//         self.height,
//         ExtendedColorType::Rgb8,
//     )
//     .map_err(|e| ImgError::Conversion {
//         source: e,
//         id: self.describe_source(),
//         format: ImageFormat::Jpeg,
//     })?;

// //png

// let mut buffer = Vec::new();
// self.img
//     .write_to(&mut Cursor::new(&mut buffer), (ImageFormat::Png).into())
//     .map_err(|e| ImgError::Conversion {
//         source: e,
//         id: self.describe_source(),
//         format: ImageFormat::Png,
//     })?;

// //webp lossless
// use webp::{Encoder, PixelLayout};
// let rgba_image = self.img.to_rgba8();
// let encoder = Encoder::new(&rgba_image, PixelLayout::Rgba, self.width, self.height);

// let webp_data = encoder.encode_lossless();

// //webp lossy
// use webp::{Encoder, PixelLayout};
// let rgba_image = self.img.to_rgba8();
// let encoder = Encoder::new(&rgba_image, PixelLayout::Rgba, self.width, self.height);

// let webp_data =
//     encoder
//         .encode_simple(false, quality as f32)
//         .map_err(|e| ImgError::WebPConversion {
//             err: e,
//             id: self.describe_source(),
//         })?;
