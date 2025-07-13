use {
    reqwest::{get, Response},
    url::Url,
};

use crate::{image::r#async::traits::UrlDownloaderOp, ImageError, Result};

pub struct UrlDownloader;

impl UrlDownloaderOp for UrlDownloader {
    async fn url(&self, url: Url) -> Result<Response> {
        get(url.clone())
            .await
            .map_err(|e| ImageError::DownloadFailed { source: e, url })
    }

    async fn parse_response(&self, response: Response) -> Result<(Vec<u8>, Url)> {
        let url = response.url().to_owned();

        if !response.status().is_success() {
            let status_code = response.status().as_u16();
            let message = response
                .text()
                .await
                .unwrap_or_else(|_| "response couldn't be read".to_string());

            return Err(ImageError::FailedRequest {
                message,
                status_code,
                url: url.clone(),
            });
        }

        let bytes = response
            .bytes()
            .await
            .map_err(|e| ImageError::ResponseReadFailed {
                source: e,
                url: url.clone(),
            })?
            .to_vec();

        Ok((bytes, url))
    }
}
