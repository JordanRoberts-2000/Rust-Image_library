use {
    crate::InnerError,
    reqwest::blocking::{get, Response},
    url::Url,
};

pub struct BlockingHttpClient;

impl BlockingHttpClient {
    pub fn fetch_url(url: impl AsRef<str>) -> Result<Response, InnerError> {
        let url = Url::parse(url.as_ref())?;
        get(url.clone()).map_err(|e| InnerError::DownloadFailed { source: e, url })
    }

    pub fn parse_response(response: Response) -> Result<Vec<u8>, InnerError> {
        let url = response.url().to_owned();

        if !response.status().is_success() {
            let status_code = response.status().as_u16();
            let message =
                response.text().unwrap_or_else(|_| "response couldn't be read".to_string());

            return Err(InnerError::FailedRequest { message, status_code, url: url.clone() });
        }

        let bytes = response
            .bytes()
            .map_err(|e| InnerError::ResponseReadFailed { source: e, url: url.clone() })?
            .to_vec();

        Ok(bytes)
    }
}
