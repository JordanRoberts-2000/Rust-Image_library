use {crate::Result, reqwest::blocking::Response, url::Url};

pub trait HttpClientOps {
    fn url(&self, url: Url) -> Result<Response>;
    fn parse_response(&self, response: Response) -> Result<(Vec<u8>, Url)>;
}
