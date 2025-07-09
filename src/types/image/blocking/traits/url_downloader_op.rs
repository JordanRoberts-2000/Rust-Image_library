use {crate::ImageError, mockall::automock, reqwest::blocking::Response, url::Url};

#[automock]
pub trait UrlDownloaderOp {
    fn url(&self, url: Url) -> Result<Response, ImageError>;
}
