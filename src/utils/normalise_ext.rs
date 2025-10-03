pub fn normalise_ext(ext: &str) -> String {
    ext.trim().trim_start_matches('.').to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::normalise_ext;

    fn case(input: &str, expected: &str) {
        let normalised = normalise_ext(input);
        assert_eq!(normalised, expected, "input: {:?}", input);
    }

    #[test]
    fn normalise_ext_basic() {
        case("PNG", "png");
        case("jpeg", "jpeg");
        case("JpG", "jpg");
    }

    #[test]
    fn normalise_ext_strips_leading_dots() {
        case(".jpg", "jpg");
        case("..jpeg", "jpeg");
        case("...PNG", "png");
        case(".bashrc", "bashrc");
    }

    #[test]
    fn normalise_ext_trims_whitespace() {
        case("  .JPG", "jpg");
        case(".JPG  ", "jpg");
        case("   png   ", "png");
        case("  .HeIc  ", "heic");
        case("   ", "");
    }

    #[test]
    fn normalise_ext_does_not_strip_trailing_dots_or_internals() {
        case("png...", "png...");
        case("tar.gz", "tar.gz");
    }

    #[test]
    fn normalise_ext_empty_and_weird_inputs() {
        case("", "");
        case(".", "");
        case("..", "");
    }
}
