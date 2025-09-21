#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Blur {
    Sm,
    Md,
    Lg,
    Xl,
    Custom(u8),
}

impl Blur {
    pub fn to_value(self) -> f32 {
        match self {
            Blur::Sm => 8.0,
            Blur::Md => 16.0,
            Blur::Lg => 32.0,
            Blur::Xl => 64.0,
            Blur::Custom(v) => (v.min(100)) as f32,
        }
    }
}

impl From<u8> for Blur {
    fn from(v: u8) -> Self {
        Blur::Custom(v)
    }
}
