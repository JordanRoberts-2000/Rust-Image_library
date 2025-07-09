use {
    crate::{
        image::blocking::{
            dependencies::ImageDeps,
            traits::{ImageDepsOps, UrlDownloaderOp},
            Image,
        },
        ImageError, Result,
    },
    url::Url,
};

impl Image {
    pub fn from_url(url: impl AsRef<str>) -> Result<Self> {
        let url = Url::parse(url.as_ref())
            .map_err(|e| ImageError::UrlParse(e, url.as_ref().to_string()))?;

        Self::from_url_internal(url, &ImageDeps::default())
    }

    pub fn from_url_internal(url: Url, image_deps: &impl ImageDepsOps) -> Result<Self> {
        let response = image_deps.download().url(url)?;
        Self::from_http_response_internal(response, image_deps)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use mockito::Server;
//     use std::fs;
//     use url::Url;

//     #[test]
//     fn test_img_download_with_mockito() {
//         let image_bytes = fs::read("tests/assets/test.png").expect("Failed to read test image");

//         let mut server = Server::new();

//         let _mock = server
//             .mock("GET", "/test.png")
//             .with_status(200)
//             .with_header("Content-Type", "image/png")
//             .with_body(image_bytes)
//             .create();

//         let url = format!("{}/test.png", server.url());

//         let img = Img::from_url(&url).expect("Should download from mock server");

//         assert_eq!(img.format, ImageFormat::Png, "Image format should be PNG");
//     }

//     #[test]
//     fn test_img_download_multiple_formats() {
//         let formats = vec![
//             ("test.png", "image/png", ImageFormat::Png),
//             ("test.webp", "image/webp", ImageFormat::WebP),
//             ("test.jpg", "image/jpeg", ImageFormat::Jpeg),
//         ];

//         for (filename, content_type, expected_format) in formats {
//             let image_bytes =
//                 fs::read(format!("tests/assets/{}", filename)).expect("Failed to read test image");

//             let mut server = Server::new();

//             let route = format!("/{}", filename);
//             let _mock = server
//                 .mock("GET", route.as_str())
//                 .with_status(200)
//                 .with_header("Content-Type", content_type)
//                 .with_body(image_bytes)
//                 .create();

//             let url = format!("{}{}", server.url(), route);
//             let img = Img::from_url(&url).expect("Should download from mock server");

//             assert_eq!(
//                 img.format, expected_format,
//                 "Image format for {} should be correct",
//                 filename
//             );
//         }
//     }

//     #[test]
//     fn test_img_download_accepts_various_params() {
//         let image_bytes = fs::read("tests/assets/test.png").unwrap();
//         let mut server = Server::new();

//         let _mock = server
//             .mock("GET", "/test.png")
//             .with_status(200)
//             .with_header("Content-Type", "image/png")
//             .with_body(image_bytes)
//             .create();

//         let string_url = format!("{}/test.png", server.url());
//         let str_url: &str = &string_url;
//         let url_obj = Url::parse(&string_url).unwrap();

//         // String
//         Img::from_url(string_url.clone()).expect("String URL should work");

//         // &str
//         Img::from_url(str_url).expect("&str URL should work");

//         // Url object
//         Img::from_url(url_obj).expect("Url object should work");
//     }

//     #[test]
//     fn test_img_download_invalid_url_should_fail() {
//         let bad_url = "ht^tp://[::invalid-url"; // clearly invalid

//         let result = Img::from_url(bad_url);
//         match result {
//             Err(ImgError::UrlParse(_, url)) => {
//                 assert_eq!(url, bad_url);
//             }
//             _ => panic!("Expected UrlParse error"),
//         }
//     }

//     #[test]
//     fn test_img_download_404_should_fail() {
//         let server = Server::new();

//         // No mock for this route = 404
//         let url = format!("{}/not-found.png", server.url());
//         let result = Img::from_url(&url);

//         match result {
//             Err(ImgError::FailedRequest {
//                 url: err_url,
//                 status_code,
//                 ..
//             }) => {
//                 assert_eq!(err_url, url);
//                 assert_eq!(status_code, 501);
//             }
//             Err(e) => panic!("Expected DownloadFailed error, got: {:#?}", e),
//             Ok(_) => panic!("Expected DownloadFailed error but got Ok"),
//         }
//     }
// }
