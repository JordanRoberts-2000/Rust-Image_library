use std::fmt;

const QUALITY_MIN: u8 = 1;
const QUALITY_MAX: u8 = 100;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Quality(u8);

impl Quality {
    pub const fn new(q: u8) -> Self {
        Self(if q < QUALITY_MIN {
            QUALITY_MIN
        } else if q > QUALITY_MAX {
            QUALITY_MAX
        } else {
            q
        })
    }

    pub const fn get(self) -> u8 {
        self.0
    }

    pub fn set(&mut self, q: u8) {
        *self = Self::new(q);
    }
}

impl fmt::Display for Quality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<u8> for Quality {
    #[inline]
    fn from(q: u8) -> Self {
        Quality::new(q)
    }
}

impl From<Quality> for u8 {
    #[inline]
    fn from(q: Quality) -> u8 {
        q.0
    }
}

impl From<Quality> for f32 {
    #[inline]
    fn from(q: Quality) -> f32 {
        q.0 as f32
    }
}
