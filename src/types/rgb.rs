use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn from_hex(hex: &str) -> Result<Self, &'static str> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return Err("Hex string must be 6 characters");
        }

        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Invalid hex")?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid hex")?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid hex")?;

        Ok(Self::new(r, g, b))
    }

    pub fn from_hsl(h: f32, s: f32, l: f32) -> Self {
        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = l - c / 2.0;

        let (r_prime, g_prime, b_prime) = match h as u32 {
            0..=59 => (c, x, 0.0),
            60..=119 => (x, c, 0.0),
            120..=179 => (0.0, c, x),
            180..=239 => (0.0, x, c),
            240..=299 => (x, 0.0, c),
            300..=359 => (c, 0.0, x),
            _ => (0.0, 0.0, 0.0),
        };

        Self::new(
            ((r_prime + m) * 255.0).round() as u8,
            ((g_prime + m) * 255.0).round() as u8,
            ((b_prime + m) * 255.0).round() as u8,
        )
    }

    pub fn rgb(&self) -> String {
        format!("rgb({}, {}, {})", self.r, self.g, self.b)
    }

    pub fn hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    pub fn luminance(&self) -> f32 {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        0.299 * r + 0.587 * g + 0.114 * b
    }

    pub fn is_dark(&self) -> bool {
        self.luminance() < 0.5
    }

    pub fn to_grayscale(&self) -> Self {
        let gray = (self.luminance() * 255.0) as u8;
        Self::new(gray, gray, gray)
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.rgb())
    }
}

impl From<[u8; 3]> for Rgb {
    fn from(arr: [u8; 3]) -> Self {
        Self::new(arr[0], arr[1], arr[2])
    }
}

impl From<(u8, u8, u8)> for Rgb {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self::new(r, g, b)
    }
}

impl From<Rgb> for [u8; 3] {
    fn from(rgb: Rgb) -> Self {
        [rgb.r, rgb.g, rgb.b]
    }
}

impl From<Rgb> for (u8, u8, u8) {
    fn from(rgb: Rgb) -> Self {
        (rgb.r, rgb.g, rgb.b)
    }
}
