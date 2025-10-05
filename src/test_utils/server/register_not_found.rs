use httpmock::{Method::GET, Mock, MockServer};

pub fn register_not_found(server: &MockServer) -> (Mock, String) {
    let body_text = "not found (custom)";

    let mock = server.mock(|when, then| {
        when.method(GET).path("/missing");
        then.status(404).header("Content-Type", "text/plain; charset=utf-8").body(body_text);
    });

    let url = format!("{}/missing", server.base_url());
    (mock, url)
}
