use {
    crate::{image::blocking::traits::UrlDownloaderOp, ImageError},
    reqwest::blocking::{get, Response},
    url::Url,
};

pub struct UrlDownloader;

impl UrlDownloaderOp for UrlDownloader {
    fn url(&self, url: Url) -> Result<Response, ImageError> {
        get(url.clone()).map_err(|e| ImageError::DownloadFailed { source: e, url })
    }
}
