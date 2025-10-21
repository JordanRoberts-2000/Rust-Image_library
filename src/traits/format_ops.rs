use {crate::utils::normalise_ext, strum::IntoEnumIterator};

pub trait FormatOps: IntoEnumIterator + Copy + Sized {
    fn mime_type(&self) -> &'static str;
    fn primary_extension(&self) -> &'static str;
    fn extensions(&self) -> &'static [&'static str];

    fn all() -> Vec<Self> {
        Self::iter().collect()
    }

    fn supported_exts() -> Vec<&'static str> {
        Self::all().iter().flat_map(|fmt| fmt.extensions().iter().copied()).collect()
    }

    fn is_supported_ext(ext: &str) -> bool {
        let normalised = normalise_ext(ext);
        Self::supported_exts().iter().any(|&e| e == normalised)
    }

    fn from_extension(ext: &str) -> Option<Self> {
        let n = normalise_ext(ext);
        Self::all().into_iter().find(|f| f.extensions().iter().any(|&e| e == n))
    }

    fn from_mime(mime: &str) -> Option<Self> {
        let m = mime.trim().to_ascii_lowercase();
        Self::all().into_iter().find(|f| f.mime_type() == m)
    }
}
