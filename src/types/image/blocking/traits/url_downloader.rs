use {crate::Result, mockall::automock, reqwest::blocking::Response, url::Url};

#[automock]
pub trait UrlDownloaderOp {
    fn url(&self, url: Url) -> Result<Response>;
    fn parse_response(&self, response: Response) -> Result<(Vec<u8>, Url)>;
}
