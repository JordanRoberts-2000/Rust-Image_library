// let mut buf = Vec::new();
// let crate_fmt: image::ImageFormat = self.format.into();
// img.write_to(&mut buf, crate_fmt)?;
// let size_bytes = buf.len();

// impl Img {
//     pub fn encoded_size(&mut self) -> Result<usize, ImageError> {
//         self.apply_transforms()?;
//         let img = self.get_decoded()?;

//         let mut buf = Vec::new();
//         let format = self.format;
//         let quality = self.config.quality.unwrap_or(default_quality_for(format));

//         write_encoded(&mut buf, img, format, quality)?; // your internal logic
//         Ok(buf.len())
//     }
// }
