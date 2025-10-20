use {
    crate::{utils::http, ImageMetadata, Result, WithSrc},
    url::Url,
};

impl ImageMetadata {
    pub fn from_url(url: impl AsRef<str>) -> Result<Self> {
        let url = Url::parse(url.as_ref())?;

        let res: Result<Self> = (|| {
            let bytes = http::download_image(&url)?;
            Self::from_bytes(&bytes)
        })();

        res.with_src(&url)
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            test_utils::{server, MOCK_IMAGE_DIMENSIONS},
            ErrorKind, ImageFormat, ImageSrc,
        },
        httpmock::MockServer,
        strum::IntoEnumIterator,
    };

    #[test]
    fn from_url_all_formats_ok() {
        let server = MockServer::start();

        for fmt in ImageFormat::iter() {
            let (mock, url) = server::register_image(&server, fmt);

            let md = ImageMetadata::from_url(&url)
                .unwrap_or_else(|e| panic!("from_url failed for {fmt:?} at {url}: {e}"));

            assert_eq!(md.format, fmt, "detected format mismatch for {fmt:?}");
            assert_eq!(md.dimensions(), MOCK_IMAGE_DIMENSIONS);
            mock.assert();
        }
    }

    #[test]
    fn from_url_404_yields_failed_request_and_keeps_src() {
        let server = MockServer::start();
        let (mock, url) = server::register_not_found(&server);

        let err = ImageMetadata::from_url(&url).expect_err("404 must error");

        match err.kind() {
            ErrorKind::FailedRequest { status_code, url: u, .. } => {
                assert_eq!(*status_code, 404);
                assert_eq!(u.as_str(), url, "error should carry failing URL");
            }
            other => panic!("expected FailedRequest {{..}}, got {:?}", other),
        }

        if let Some(src) = err.src() {
            assert!(matches!(src, ImageSrc::Url(u) if u.as_str() == url));
        }

        mock.assert();
    }

    #[test]
    fn from_url_invalid_url_parse_err() {
        let bad = "not a url";
        let res = ImageMetadata::from_url(bad);
        assert!(res.is_err(), "invalid URL should error");
    }

    #[test]
    fn from_url_corrupted_payload_dimensions_or_guess_err() {
        let server = MockServer::start();
        let (mock, url) = server::register_corrupted_header_image(&server, ImageFormat::Png);

        let err =
            ImageMetadata::from_url(&url).expect_err("corrupted PNG payload via HTTP should fail");

        assert!(
            matches!(
                err.kind(),
                ErrorKind::PeakDimensionsFailed(_) | ErrorKind::FormatDetectionFailed(_)
            ),
            "unexpected error kind: {:?}",
            err.kind()
        );

        if let Some(src) = err.src() {
            assert!(matches!(src, ImageSrc::Url(u) if u.as_str() == url));
        }

        mock.assert();
    }

    #[test]
    fn from_url_network_error_maps_to_download_failed_and_keeps_src() {
        let url = "http://nonexistent.invalid/some/path";
        let err = ImageMetadata::from_url(url).expect_err("unreachable host should error");

        match err.kind() {
            ErrorKind::DownloadFailed { url: u, .. } => assert_eq!(u.as_str(), url),
            other => panic!("expected DownloadFailed {{..}}, got {:?}", other),
        }

        if let Some(src) = err.src() {
            assert!(matches!(src, ImageSrc::Url(u) if u.as_str() == url));
        }
    }
}
