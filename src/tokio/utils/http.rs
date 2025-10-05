use {
    crate::{ErrorKind, Result},
    reqwest::{get, Response},
    url::Url,
};

pub async fn download_image(url: impl AsRef<str>) -> Result<Vec<u8>> {
    let url = Url::parse(url.as_ref())?;
    let response = get(url.clone())
        .await
        .map_err(|e| ErrorKind::DownloadFailed { source: e, url: url.clone() })?;

    parse_response(response).await
}

async fn parse_response(response: Response) -> Result<Vec<u8>> {
    let url = response.url().to_owned();

    if !response.status().is_success() {
        let status_code = response.status().as_u16();
        let message =
            response.text().await.unwrap_or_else(|_| "response couldn't be read".to_string());

        return Err(ErrorKind::FailedRequest { message, status_code, url: url.clone() }.into());
    }

    let bytes = response
        .bytes()
        .await
        .map_err(|e| ErrorKind::ResponseReadFailed { source: e, url: url.clone() })?
        .to_vec();

    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            test_utils::{encoded_bytes, server},
            ErrorKind, ImageFormat,
        },
        httpmock::MockServer,
        strum::IntoEnumIterator,
    };

    #[tokio::test]
    async fn download_image_ok_returns_bytes() {
        let server = MockServer::start();

        for format in ImageFormat::iter() {
            let (mock, url) = server::register_image(&server, format);

            let bytes = download_image(&url)
                .await
                .unwrap_or_else(|e| panic!("download_image failed for {format:?}: {e}"));

            assert_eq!(bytes, encoded_bytes(format), "returned bytes should match server body");
            mock.assert();
        }
    }

    #[tokio::test]
    async fn download_image_404_yields_failed_request() {
        let server = MockServer::start();
        let (mock, url) = server::register_not_found(&server);

        let err = download_image(&url).await.expect_err("404 must error");

        match err.kind() {
            ErrorKind::FailedRequest { status_code, .. } => {
                assert_eq!(*status_code, 404);
            }
            other => panic!("expected FailedRequest, got {:?}", other),
        }

        mock.assert();
    }

    #[tokio::test]
    async fn download_image_invalid_url_parse_err() {
        let bad = "not a url";
        let res = download_image(bad).await;
        assert!(res.is_err(), "invalid URL should error");
    }

    #[tokio::test]
    async fn download_image_network_error_maps_to_download_failed() {
        let raw = "http://nonexistent.invalid/some/path";

        let err = download_image(raw).await.expect_err("unreachable host should error");

        match err.kind() {
            ErrorKind::DownloadFailed { url, .. } => {
                assert_eq!(url.as_str(), raw);
            }
            other => panic!("expected DownloadFailed {{ .. }}, got {:?}", other),
        }
    }
}
