use {
    crate::{
        test_utils::{corrupted_header_bytes, encoded_bytes},
        ImageFormat,
    },
    httpmock::{Method::GET, Mock, MockServer},
};

pub fn register_image(server: &MockServer, format: ImageFormat) -> (Mock, String) {
    let payload = encoded_bytes(format);
    let path = format!("/img.{}", format.primary_extension());

    let mock = server.mock(|when, then| {
        when.method(GET).path(&path);
        then.status(200).header("Content-Type", format.mime_type()).body(payload);
    });

    let url = format!("{}{}", server.base_url(), path);
    (mock, url)
}

pub fn register_corrupted_header_image(server: &MockServer, format: ImageFormat) -> (Mock, String) {
    let payload = corrupted_header_bytes(format);
    let path = format!("/img.{}", format.primary_extension());

    let mock = server.mock(|when, then| {
        when.method(GET).path(&path);
        then.status(200).header("Content-Type", format.mime_type()).body(payload);
    });

    let url = format!("{}{}", server.base_url(), path);
    (mock, url)
}
