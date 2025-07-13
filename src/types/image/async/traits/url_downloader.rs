use {crate::Result, reqwest::Response, url::Url};

pub trait UrlDownloaderOp {
    async fn url(&self, url: Url) -> Result<Response>;
    async fn parse_response(&self, response: Response) -> Result<(Vec<u8>, Url)>;
}
