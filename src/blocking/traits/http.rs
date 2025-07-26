#[cfg(test)]
use mockall::automock;
use {crate::Result, reqwest::blocking::Response, url::Url};

#[cfg_attr(test, automock)]
pub trait HttpClientOps {
    fn url(&self, url: Url) -> Result<Response>;
    fn parse_response(&self, response: Response) -> Result<(Vec<u8>, Url)>;
}
