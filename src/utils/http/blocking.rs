use {
    crate::{ImageError, Result},
    reqwest::blocking::{get, Response},
    url::Url,
};

pub struct BlockingHttpClient;

impl BlockingHttpClient {
    pub fn fetch_url(url: impl AsRef<str>) -> Result<Response> {
        let url = Url::parse(url.as_ref())?;
        get(url.clone()).map_err(|e| ImageError::DownloadFailed { source: e, url })
    }

    pub fn parse_response(response: Response) -> Result<Vec<u8>> {
        let url = response.url().to_owned();

        if !response.status().is_success() {
            let status_code = response.status().as_u16();
            let message =
                response.text().unwrap_or_else(|_| "response couldn't be read".to_string());

            return Err(ImageError::FailedRequest { message, status_code, url: url.clone() });
        }

        let bytes = response
            .bytes()
            .map_err(|e| ImageError::ResponseReadFailed { source: e, url: url.clone() })?
            .to_vec();

        Ok(bytes)
    }
}
