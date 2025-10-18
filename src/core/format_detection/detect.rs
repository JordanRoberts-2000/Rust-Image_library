pub fn svg(bytes: &[u8]) -> bool {
    let text = match str::from_utf8(bytes) {
        Ok(t) => t,
        Err(_) => return false,
    };

    let trimmed = text.trim_start();

    if trimmed.starts_with("<svg") {
        return true;
    }

    if trimmed.starts_with("<?xml") {
        return text.contains("<svg");
    }

    false
}

pub fn pdf(bytes: &[u8]) -> bool {
    if bytes.starts_with(b"%PDF-") {
        return true;
    }
    // Tolerant path: find "%PDF-" somewhere near the start
    bytes.windows(5).any(|w| w == b"%PDF-")
}
