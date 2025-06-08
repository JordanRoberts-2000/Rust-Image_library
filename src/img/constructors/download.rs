use std::path::PathBuf;

use image::{guess_format, GenericImageView};
use reqwest::blocking;
use url::Url;

use crate::{constants::DEFAULT_FILENAME, enums::ImgSrc, Img, ImgError, Result};

impl Img {
    pub fn download<T: AsRef<str>>(url: T) -> Result<Self> {
        let raw = url.as_ref();

        let parsed_url = Url::parse(raw).map_err(|e| ImgError::UrlParseFailed {
            url: raw.to_string(),
            source: e,
        })?;

        let response = blocking::get(parsed_url.clone()).map_err(|e| ImgError::DownloadFailed {
            source: e,
            url: raw.to_string(),
        })?;

        if !response.status().is_success() {
            let status_code = response.status().as_u16();
            let message = response
                .text()
                .unwrap_or_else(|_| "response couldn't be read".to_string());

            return Err(ImgError::FailedRequest {
                message,
                status_code,
                url: raw.to_string(),
            });
        }

        let bytes = response.bytes().map_err(|e| ImgError::ResponseReadFailed {
            source: e,
            url: raw.to_string(),
        })?;

        let format = guess_format(&bytes).map_err(|_| ImgError::GuessFormat)?;
        let size_bytes = bytes.len();

        let img = image::load_from_memory(&bytes).map_err(|e| ImgError::Decoding {
            id: raw.to_string(),
            source: e,
            format,
        })?;

        let (width, height) = img.dimensions();

        let ext = match format.extensions_str().first() {
            Some(ext) => ext,
            None => return Err(ImgError::ExtensionInvalid),
        };
        let file_name = format!("{}.{}", DEFAULT_FILENAME, ext);

        let cwd = PathBuf::from(".");
        let target = cwd.join(&file_name);

        Ok(Self {
            img,
            src: ImgSrc::Remote { url: parsed_url },
            target_path: target,
            height,
            width,
            aspect_ratio: width as f32 / height as f32,
            format,
            size_bytes,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use std::fs;
    use url::Url;

    #[test]
    fn test_img_download_with_mockito() {
        let image_bytes = fs::read("tests/assets/test.png").expect("Failed to read test image");

        let mut server = Server::new();

        let _mock = server
            .mock("GET", "/test.png")
            .with_status(200)
            .with_header("Content-Type", "image/png")
            .with_body(image_bytes)
            .create();

        let url = format!("{}/test.png", server.url());

        let img = Img::download(&url).expect("Should download from mock server");

        assert_eq!(
            img.format,
            image::ImageFormat::Png,
            "Image format should be PNG"
        );
    }

    #[test]
    fn test_img_download_multiple_formats() {
        use image::ImageFormat;

        let formats = vec![
            ("test.png", "image/png", ImageFormat::Png),
            ("test.webp", "image/webp", ImageFormat::WebP),
            ("test.jpg", "image/jpeg", ImageFormat::Jpeg),
        ];

        for (filename, content_type, expected_format) in formats {
            let image_bytes =
                fs::read(format!("tests/assets/{}", filename)).expect("Failed to read test image");

            let mut server = Server::new();

            let route = format!("/{}", filename);
            let _mock = server
                .mock("GET", route.as_str())
                .with_status(200)
                .with_header("Content-Type", content_type)
                .with_body(image_bytes)
                .create();

            let url = format!("{}{}", server.url(), route);
            let img = Img::download(&url).expect("Should download from mock server");

            assert_eq!(
                img.format, expected_format,
                "Image format for {} should be correct",
                filename
            );
        }
    }

    #[test]
    fn test_img_download_accepts_various_params() {
        let image_bytes = fs::read("tests/assets/test.png").unwrap();
        let mut server = Server::new();

        let _mock = server
            .mock("GET", "/test.png")
            .with_status(200)
            .with_header("Content-Type", "image/png")
            .with_body(image_bytes)
            .create();

        let string_url = format!("{}/test.png", server.url());
        let str_url: &str = &string_url;
        let url_obj = Url::parse(&string_url).unwrap();

        // String
        Img::download(string_url.clone()).expect("String URL should work");

        // &str
        Img::download(str_url).expect("&str URL should work");

        // Url object
        Img::download(url_obj).expect("Url object should work");
    }

    #[test]
    fn test_img_download_invalid_url_should_fail() {
        let bad_url = "ht^tp://[::invalid-url"; // clearly invalid

        let result = Img::download(bad_url);
        match result {
            Err(ImgError::UrlParseFailed { url, .. }) => {
                assert_eq!(url, bad_url);
            }
            _ => panic!("Expected UrlParseFailed error"),
        }
    }

    #[test]
    fn test_img_download_404_should_fail() {
        let server = Server::new();

        // No mock for this route = 404
        let url = format!("{}/not-found.png", server.url());
        let result = Img::download(&url);

        match result {
            Err(ImgError::FailedRequest {
                url: err_url,
                status_code,
                ..
            }) => {
                assert_eq!(err_url, url);
                assert_eq!(status_code, 501);
            }
            Err(e) => panic!("Expected DownloadFailed error, got: {:#?}", e),
            Ok(_) => panic!("Expected DownloadFailed error but got Ok"),
        }
    }
}
