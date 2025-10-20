use {
    crate::{
        image::{utils::decode, ImageConfig},
        utils::http,
        ErrorKind, Format, Image, ImageSrc, Result, WithSrc,
    },
    std::{
        cell::RefCell,
        io::{BufReader, Cursor},
    },
    url::Url,
};

impl Image {
    pub fn from_url(url: impl AsRef<str>) -> Result<Self> {
        let src = ImageSrc::Url(url.as_ref().to_string());

        (|| -> Result<Self> {
            let url = Url::parse(url.as_ref())?;
            let bytes = http::download_image(&url)?;

            let mut reader = BufReader::new(Cursor::new(bytes));
            let format = Format::guess_from_reader(&mut reader)?.ok_or(ErrorKind::UnknownFormat)?;
            let decoded = decode::from_reader(&mut reader, &format)?;

            Ok(Self {
                src: src.clone(),
                decoded: RefCell::new(decoded),
                config: ImageConfig::default(),
                format: Some(format),
            })
        })()
        .with_src(src)
    }
}

// #[cfg(test)]
// mod tests {
//     use {
//         super::*,
//         crate::{test_utils::encoded_bytes, ImageFormat},
//         httpmock::prelude::*,
//         strum::IntoEnumIterator,
//         url::Url,
//     };

//     #[test]
//     fn from_url_happy_path_sets_fields_and_preserves_bytes() -> Result<()> {
//         let server = MockServer::start();

//         for format in ImageFormat::iter() {
//             let bytes = encoded_bytes(format);

//             let _m = server.mock(|when, then| {
//                 when.method(GET).path("/img.png");
//                 then.status(200).header("content-type", "image/png").body(bytes.clone());
//             });

//             let url = Url::parse(&format!("{}/img.png", server.base_url())).unwrap();
//             let img = Image::from_url(&url)?;

//             match &img.src {
//                 ImageSrc::Url(u) => assert_eq!(u, &url),
//                 _ => panic!("expected ImageSrc::Url"),
//             }

//             {
//                 let data = img.data.borrow();
//                 match &*data {
//                     ImageData::EncodedBytes(b) => assert_eq!(b, &bytes),
//                     _ => panic!("expected ImageData::EncodedBytes"),
//                 }
//             }
//         }

//         Ok(())
//     }

//     #[test]
//     fn from_url_rejects_non_image_payload() {
//         let server = MockServer::start();
//         let _m = server.mock(|when, then| {
//             when.method(GET).path("/text");
//             then.status(200).header("content-type", "text/plain").body("hello world");
//         });

//         let url = Url::parse(&format!("{}/text", server.base_url())).unwrap();
//         let res = Image::from_url(&url);
//         assert!(res.is_err(), "expected failure for non-image payload");
//     }
// }
