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
mod serializers;
mod metadata {
    mod color;
    mod dimensions;
    mod format;
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
mod saving {
    mod save;
    mod save_as;
    mod save_to_folder;
}
