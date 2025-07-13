mod constructors {
    mod from_base64;
    mod from_bytes;
    mod from_file;
    mod from_http_response;
    mod from_raw_pixels;
    mod from_raw_reader;
    mod from_reader;
    mod from_url;
}
mod serialization;
mod metadata {
    mod color;
    mod dimensions;
    mod size;
}
mod internal;
mod source {
    mod remove_source_file;
    mod source;
}
mod configure;
mod encoding;
mod output;
mod transformations;
