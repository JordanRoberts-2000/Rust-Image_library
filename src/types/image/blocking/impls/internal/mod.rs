mod apply_transforms;
mod atomic;
mod decoding {
    mod decode;
    mod get_decoded;
}
mod describe_source;
mod encoding {
    mod encode;
    mod formats {
        mod avif;
        mod jpeg {
            mod progressive;
            mod standard;
        }
        mod png;
        mod webp;
    }
    mod write_encoded;
}
