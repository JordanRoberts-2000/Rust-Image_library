use std::fmt;

const SPEED_MIN: u8 = 1;
const SPEED_MAX: u8 = 30;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct GifSpeed(u8);

impl GifSpeed {
    pub const fn new(q: u8) -> Self {
        Self(if q < SPEED_MIN {
            SPEED_MIN
        } else if q > SPEED_MAX {
            SPEED_MAX
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

impl fmt::Display for GifSpeed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<u8> for GifSpeed {
    #[inline]
    fn from(q: u8) -> Self {
        GifSpeed::new(q)
    }
}

impl From<GifSpeed> for u8 {
    #[inline]
    fn from(q: GifSpeed) -> u8 {
        q.0
    }
}

impl From<GifSpeed> for i32 {
    #[inline]
    fn from(q: GifSpeed) -> i32 {
        q.0 as i32
    }
}
