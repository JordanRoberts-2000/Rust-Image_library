// use {
//     base64::{engine::general_purpose::STANDARD, Engine},
//     std::io::Cursor,
// };

// use crate::{Image, ImageError, Result};

// impl Image {
//     pub fn to_data_url(&mut self) -> Result<String> {
//         let mut buffer = Cursor::new(Vec::new());
//         self.raw
//             .write_to(&mut buffer, self.format.into())
//             .map_err(|e| ImageError::Conversion {
//                 source: e,
//                 id: self.describe_source(),
//                 format: self.format,
//             })?;

//         let encoded = STANDARD.encode(buffer.get_ref());
//         let data_url = format!("data:{};base64,{}", self.format.to_mime_type(), encoded);

//         Ok(data_url)
//     }
// }
