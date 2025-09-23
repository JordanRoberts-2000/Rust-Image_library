use {
    crate::InnerError,
    reqwest::{get, Response},
    url::Url,
};

pub async fn download_image(url: impl AsRef<str>) -> Result<Vec<u8>, InnerError> {
    let url = Url::parse(url.as_ref())?;
    let response = get(url.clone())
        .await
        .map_err(|e| InnerError::DownloadFailed { source: e, url: url.clone() })?;

    parse_response(response).await
}

async fn parse_response(response: Response) -> Result<Vec<u8>, InnerError> {
    let url = response.url().to_owned();

    if !response.status().is_success() {
        let status_code = response.status().as_u16();
        let message =
            response.text().await.unwrap_or_else(|_| "response couldn't be read".to_string());

        return Err(InnerError::FailedRequest { message, status_code, url: url.clone() });
    }

    let bytes = response
        .bytes()
        .await
        .map_err(|e| InnerError::ResponseReadFailed { source: e, url: url.clone() })?
        .to_vec();

    Ok(bytes)
}
