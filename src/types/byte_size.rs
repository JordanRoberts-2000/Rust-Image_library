use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ByteSize(usize);

impl ByteSize {
    pub fn new<T: Into<usize>>(size: T) -> Self {
        ByteSize(size.into())
    }

    pub fn bytes(self) -> usize {
        self.0
    }

    pub fn kib(self) -> usize {
        self.0 / 1024
    }

    pub fn mib(self) -> usize {
        self.0 / (1024 * 1024)
    }

    pub fn gib(self) -> usize {
        self.0 / (1024 * 1024 * 1024)
    }

    pub fn kb(self) -> usize {
        self.0 / 1000
    }

    pub fn mb(self) -> usize {
        self.0 / 1_000_000
    }

    pub fn gb(self) -> usize {
        self.0 / 1_000_000_000
    }

    /// Display as decimal string (e.g. "1.23MB")
    pub fn to_decimal_string(&self) -> String {
        let bytes = self.0 as f64;

        if bytes >= 1_000_000_000.0 {
            format!("{:.2}GB", bytes / 1_000_000_000.0)
        } else if bytes >= 1_000_000.0 {
            format!("{:.2}MB", bytes / 1_000_000.0)
        } else if bytes >= 1_000.0 {
            format!("{:.2}KB", bytes / 1_000.0)
        } else {
            format!("{:.0}B", bytes)
        }
    }

    /// Display as binary string (e.g. "1.23MiB")
    pub fn to_binary_string(&self) -> String {
        let bytes = self.0 as f64;

        if bytes >= 1024.0 * 1024.0 * 1024.0 {
            format!("{:.2}GiB", bytes / (1024.0 * 1024.0 * 1024.0))
        } else if bytes >= 1024.0 * 1024.0 {
            format!("{:.2}MiB", bytes / (1024.0 * 1024.0))
        } else if bytes >= 1024.0 {
            format!("{:.2}KiB", bytes / 1024.0)
        } else {
            format!("{:.0}B", bytes)
        }
    }
}

impl fmt::Display for ByteSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_decimal_string())
    }
}

macro_rules! impl_from_for_bytesize {
  ($($t:ty),*) => {
      $(
          impl From<$t> for ByteSize {
              fn from(value: $t) -> Self {
                  ByteSize(value as usize)
              }
          }
      )*
  };
}

impl_from_for_bytesize!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);

impl From<ByteSize> for usize {
    fn from(size: ByteSize) -> usize {
        size.0
    }
}
