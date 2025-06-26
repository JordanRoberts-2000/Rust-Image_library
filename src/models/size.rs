use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ImageSize(usize);

impl ImageSize {
    pub fn new<T: Into<usize>>(size: T) -> Self {
        ImageSize(size.into())
    }

    pub fn bytes(self) -> usize {
        self.0
    }

    pub fn kb(self) -> usize {
        self.0 / 1024
    }

    pub fn mb(self) -> usize {
        self.0 / (1024 * 1024)
    }
}

impl fmt::Display for ImageSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = self.0 as f64;

        if bytes >= 1_000_000.0 {
            let mb = (bytes / 1_000_000.0 * 100.0).floor() / 100.0;
            write!(f, "{:.2}MB", mb)
        } else if bytes >= 1_000.0 {
            let kb = (bytes / 1_000.0 * 100.0).floor() / 100.0;
            write!(f, "{:.2}KB", kb)
        } else {
            write!(f, "{:.0}B", bytes.floor())
        }
    }
}

impl From<usize> for ImageSize {
    fn from(size: usize) -> Self {
        ImageSize(size)
    }
}

impl From<ImageSize> for usize {
    fn from(size: ImageSize) -> usize {
        size.0
    }
}
