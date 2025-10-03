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
